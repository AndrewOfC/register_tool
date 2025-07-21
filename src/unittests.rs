// 
// SPDX-License-Identifier: MIT
// 
// Copyright (c) 2025 Andrew Ellis Page
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
// 
#[cfg(test)]
pub mod rtool_tests {
    use std::io::Write;
    use std::process;
    use aep_rust_common::descender::Descender;
    use aep_rust_common::yaml_descender::YamlDescender;
    use crate::register_op::parse_bits;
    use crate::register_tool::RegisterTool;

    #[test]
    fn test_bitmask() {
        let (mask, lo) = parse_bits("4:2").unwrap();
        assert_eq!(mask, 0b011100);
        assert_eq!(lo, 2);
        
        let (mask, lo) = parse_bits("3:3").unwrap() ;
        assert_eq!(mask, 0b01000) ;
        assert_eq!(lo, 3);

        let (mask, lo) = parse_bits("31:0").unwrap() ;
        assert_eq!(mask, 0xFFFFFFFF) ;
        assert_eq!(lo, 0);
    }
    #[test]
    fn test_gather_and_apply_regsters() {
        let regspecs = vec!["GPIO.words.function2=0", "GPIO.pins@27.function=1", "GPIO.words.function2", "GPIO.pins@27.function"] ;

        let working_dir = env!("CARGO_MANIFEST_DIR");
        let config_file = format!("{}/register_tool.yaml", working_dir);

        let descender = Box::new(YamlDescender::new_from_file(&*config_file, false).unwrap()) as Box<dyn Descender<dyn Write>> ;
        let mut register_tool = RegisterTool::new(descender);

        register_tool.gather_regs(&regspecs) ;
        register_tool.set_test_area() ;

        let replies = register_tool.apply_registers(|v| {
            Ok(v)
        }).expect("TODO: panic message");

        assert_eq!(replies.len(), 4);
        assert_eq!(replies[0].clone().unwrap(), 0x00000000);
        assert_eq!(replies[1].clone().unwrap(), 0x00000001);
        assert_eq!(replies[2].clone().unwrap(), 0x00200000);
        assert_eq!(replies[3].clone().unwrap(), 0x00000001);
    }

    
}