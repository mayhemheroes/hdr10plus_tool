#![no_main]

use std::path::PathBuf;

use anyhow::Result;
use hevc_parser::hevc::NALUnit;
use hevc_parser::io::processor::{HevcProcessor, HevcProcessorOpts};
use hevc_parser::io::{IoFormat, IoProcessor};
use hevc_parser::HevcParser;
use libfuzzer_sys::fuzz_target;

struct FuzzProcessor {
    path: PathBuf,
}

impl IoProcessor for FuzzProcessor {
    fn input(&self) -> &PathBuf {
        &self.path
    }

    fn update_progress(&mut self, _delta: u64) {}

    fn process_nals(
        &mut self,
        _parser: &HevcParser,
        nals: &[NALUnit],
        chunk: &[u8],
    ) -> Result<()> {
        for nal in nals {
            if nal.nal_type == hevc_parser::hevc::NAL_SEI_PREFIX {
                let sei_payload = hevc_parser::utils::clear_start_code_emulation_prevention_3_byte(
                    &chunk[nal.start..nal.end],
                );
                if let Ok(msgs) = hevc_parser::hevc::SeiMessage::parse_sei_rbsp(&sei_payload) {
                    for msg in msgs {
                        if msg.payload_type == hevc_parser::hevc::USER_DATA_REGISTERED_ITU_T_35
                            && msg.payload_size >= 7
                        {
                            let start = msg.payload_offset;
                            let end = start + msg.payload_size;
                            let bytes = hevc_parser::utils::clear_start_code_emulation_prevention_3_byte(
                                &sei_payload[start..end],
                            );
                            let _ = hdr10plus::metadata::Hdr10PlusMetadata::parse(&bytes);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn finalize(&mut self, _parser: &HevcParser) -> Result<()> {
        Ok(())
    }
}

fuzz_target!(|data: &[u8]| {
    let _ = hdr10plus::metadata::Hdr10PlusMetadata::parse(data);

    let path = PathBuf::from("/tmp/fuzz.hevc");
    if std::fs::write(&path, data).is_err() {
        return;
    }

    let format = hevc_parser::io::format_from_path(&path).unwrap_or(IoFormat::Raw);
    let file_path = path.clone();
    let mut processor = FuzzProcessor { path };
    let opts = HevcProcessorOpts {
        parse_nals: true,
        ..Default::default()
    };
    let mut hevc = HevcProcessor::new(format, opts, 100_000);
    let _ = hevc.process_file(&mut processor, Some(file_path));
});
