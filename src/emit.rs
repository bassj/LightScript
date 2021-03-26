use webassembly;
use webassembly::TypeWasmExt;

trait Encoded {
    fn encode(&self) -> Vec<u8>;
}

#[derive(Clone)]
struct EncodedString {
    val: String
}

impl Encoded for EncodedString {
    fn encode(&self) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();
        
        let bytes= self.val.as_bytes();
        
        encoded.extend_from_slice(&bytes.len().to_wasm_bytes());
        encoded.extend_from_slice(bytes);

        encoded
    }
}

struct EncodedVec<E: Encoded> {
    vector: Vec<E>
}

impl<E: Encoded> Encoded for EncodedVec<E> {
    fn encode(&self) -> Vec<u8> {
        let len = self.vector.len() as u32;

        let mut encoded: Vec<u8> = Vec::new();

        encoded.extend_from_slice(&len.to_wasm_bytes());
        
        for element in self.vector.iter() {
            encoded.extend_from_slice(&element.encode());
        }
        
        encoded
    }
}

struct EncodedSection {
    section_type: u8,
    section_content: Vec<u8>
}

impl Encoded for EncodedSection {
    fn encode(&self) -> Vec<u8> {
        println!("Encoding section: {}", self.section_type);

        let content_size = self.section_content.len();

        println!("Section Size: {}", content_size);
        println!("Section Content: {:?}", self.section_content);

        let mut section: Vec<u8> = Vec::new();
        section.push(self.section_type);
        section.extend_from_slice(&content_size.to_wasm_bytes());
        section.extend_from_slice(&self.section_content);
        section
    }
}

#[derive(Clone)]
struct CodeBody {
    locals: Vec<CodeLocal>,
    instructions: Vec<u8>,
}

impl Encoded for CodeBody {
    fn encode(&self) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();

        let mut content = Vec::new();

        let locals = EncodedVec {
            vector: self.locals.to_vec()
        };

        content.extend_from_slice(&locals.encode());
        content.extend_from_slice(self.instructions.as_ref());
        content.push(webassembly::END);

        encoded.extend_from_slice(&content.len().to_wasm_bytes());
        encoded.extend_from_slice(&content);

        encoded
    }
}

#[derive(Clone, Copy)]
struct CodeLocal {
    count: usize,
    local_type: u8,
}

impl Encoded for CodeLocal {
    fn encode(&self) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();

        encoded.extend_from_slice(&(self.count as u32).to_wasm_bytes());
        encoded.push(self.local_type);

        encoded
    }
}

struct EncodedModule {
    custom_section: Option<EncodedSection>,
    type_section: Option<TypeSection>,
    import_section: Option<EncodedSection>,
    function_section: Option<FuncSection>,
    table_section: Option<EncodedSection>,
    memory_section: Option<MemSection>,
    global_section: Option<EncodedSection>,
    export_section: Option<ExportSection>,
    start_section: Option<EncodedSection>,
    element_section: Option<EncodedSection>,
    code_section: Option<CodeSection>,
    data_section: Option<EncodedSection>,
    data_count_section: Option<EncodedSection>
}

impl Encoded for EncodedModule {
    fn encode(&self) -> Vec<u8> {
        let mut content: Vec<u8> = Vec::new();

        content.extend_from_slice(webassembly::MAGIC_NUMBER);
        content.extend_from_slice(webassembly::VERSION_1);

        if self.type_section.is_some() {
            content.extend_from_slice(
                &self.type_section.as_ref().unwrap().encode()
            );
        }

        if self.function_section.is_some() {
            content.extend_from_slice(
                &self.function_section.as_ref().unwrap().encode()
            );
        }

        if self.memory_section.is_some() {
            content.extend_from_slice(
                &self.memory_section.as_ref().unwrap().encode()
            );
        }

        if self.export_section.is_some() {
            content.extend_from_slice(
                &self.export_section.as_ref().unwrap().encode()
            );
        }

        if self.code_section.is_some() {
            content.extend_from_slice(
                &self.code_section.as_ref().unwrap().encode()
            );
        }

        content
    }
}

impl Default for EncodedModule {
    fn default() -> Self {
        Self {
            custom_section: None,
            type_section: None,
            import_section: None,
            function_section: None,
            table_section: None,
            memory_section: None,
            global_section: None,
            export_section: None,
            start_section: None,
            element_section: None,
            code_section: None,
            data_section: None,
            data_count_section: None
        }
    }
}

#[derive(Clone, Debug)]
struct TypeSignature {
    type_sig: u8,
    inputs: Option<Vec<TypeSignature>>,
    outputs: Option<Vec<TypeSignature>>,
    min: Option<u32>,
    max: Option<u32>,
}

impl Default for TypeSignature {
    fn default() -> Self {
        Self {
            type_sig: 0,
            inputs: None,
            outputs: None,
            min: None,
            max: None,
        }
    }
}

impl Encoded for TypeSignature {
    fn encode(&self) -> Vec<u8> {
        let mut content: Vec<u8> = Vec::new();
        content.push(self.type_sig);

        match self.type_sig {
            webassembly::FUNC => {
                //We're encoding a function type.
                let inputs = self.inputs.as_ref().unwrap();
                let outputs = self.outputs.as_ref().unwrap();

                let encoded_inputs = EncodedVec {
                    vector: inputs.to_vec()
                };

                content.extend_from_slice(&encoded_inputs.encode());

                let encoded_outputs = EncodedVec {
                    vector: outputs.to_vec()
                };

                content.extend_from_slice(&encoded_outputs.encode());
                
                content
            },
            webassembly::LIMIT_MIN_MAX => {
                let min: u32 = self.min.unwrap();
                let max: u32 = self.max.unwrap();

                content.extend_from_slice(&min.to_wasm_bytes());
                content.extend_from_slice(&max.to_wasm_bytes());

                content
            },
            _ => {
                content
            }
        }
    }
}

struct TypeSection {
    signatures: Vec<TypeSignature>,
}

impl Encoded for TypeSection {
    fn encode(&self) -> Vec<u8> {
        EncodedSection {
            section_type: webassembly::SECTION_TYPE,
            section_content: EncodedVec {
                vector: self.signatures.to_owned()
            }.encode()
        }.encode()
    }
}

struct FuncSection {
    signature_ids: Vec<u32>
}

impl Encoded for FuncSection {
    fn encode(&self) -> Vec<u8> {
        let mut encoded_ids: Vec<u8> = Vec::new();
        encoded_ids.extend_from_slice(&self.signature_ids.len().to_wasm_bytes());
        for id in self.signature_ids.iter() {
            encoded_ids.extend_from_slice(&id.to_wasm_bytes());
        }

        let encoded_section = EncodedSection {
            section_type: webassembly::SECTION_FUNCTION,
            section_content: encoded_ids,
        };

        encoded_section.encode()
    }
}

struct MemSection {
    blocks: Vec<TypeSignature>,
}

impl Encoded for MemSection {
    fn encode(&self) -> Vec<u8> {
        let blocks = self.blocks.to_vec();

        EncodedSection {
            section_type: webassembly::SECTION_MEMORY,
            section_content: EncodedVec {
                vector: blocks
            }.encode()
        }.encode()
    } 
}

#[derive(Clone)]
struct ExportSignature {
    name: EncodedString,
    sig_type: u8,
    index: u32,
}

impl Encoded for ExportSignature {
    fn encode(&self) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();
        
        encoded.extend_from_slice(&self.name.encode());
        encoded.push(self.sig_type);
        encoded.extend_from_slice(&self.index.to_wasm_bytes());

        encoded
    }
}

struct ExportSection {
    exports: Vec<ExportSignature>
}

impl Encoded for ExportSection {
    fn encode(&self) -> Vec<u8> {
        EncodedSection {
            section_type: webassembly::SECTION_EXPORT,
            section_content: EncodedVec {
                vector: self.exports.to_vec()
            }.encode()
        }.encode()
    }
}

struct CodeSection {
    blocks: Vec<CodeBody>
}

impl Encoded for CodeSection {
    fn encode(&self) -> Vec<u8> {
        EncodedSection {
            section_type: webassembly::SECTION_CODE,
            section_content: EncodedVec {
                vector: self.blocks.to_vec()
            }.encode()
        }.encode()
    }
}

pub struct ModuleEmitter;

impl ModuleEmitter {
    pub fn emit(&self) -> Vec<u8> {
        let mut module = EncodedModule::default();
        // For right now we're just gonna build a simple module that exports a add function that accepts 2 f32's and returns 1 f32
        
        // Lets build the func section.
        let func_section = FuncSection {
            // The signature id is a vector full of pointers to a type signature, representing the signature of the function at that index.
            signature_ids: vec![0] // Our add function will be the only function, so just pick the first signature.
        };

        module.function_section = Some(func_section);

        // Lets build the type section.
        let type_section = TypeSection {
            signatures: vec![
                TypeSignature {
                    type_sig: webassembly::FUNC,
                    inputs: Some(vec![
                        TypeSignature {
                            type_sig: webassembly::F32,
                            .. TypeSignature::default()
                        },
                        TypeSignature {
                            type_sig: webassembly::F32,
                            .. TypeSignature::default()
                        },
                    ]),
                    outputs: Some(vec![
                        TypeSignature {
                            type_sig: webassembly::F32,
                            .. TypeSignature::default()
                        }
                    ]),
                    .. TypeSignature::default()
                }
            ]
        };

        module.type_section = Some(type_section);

        //Lets build ourselves a memory section.
        let mem_section = MemSection {
            blocks: vec![
                TypeSignature {
                    type_sig: webassembly::LIMIT_MIN_MAX,
                    min: Some(2), // Allocate between 2 Ki and 5 Ki
                    max: Some(10),
                    .. TypeSignature::default()
                }
            ]
        };

        module.memory_section = Some(mem_section);


        // Define our exports.

        let export_section = ExportSection {
            exports: vec![
                ExportSignature {
                    name: EncodedString {
                        val: "memory".to_string()
                    },
                    sig_type: webassembly::DESC_MEMORY,
                    index: 0
                },
                ExportSignature {
                    name: EncodedString {
                        val: "add".to_string()
                    },
                    sig_type: webassembly::DESC_FUNCTION,
                    index: 0
                }
            ]
        };

        module.export_section = Some(export_section);

        let instructions = vec![
            webassembly::LOCAL_GET, 0,
            webassembly::LOCAL_GET, 1,
            webassembly::F32_ADD
        ];
        //instructions.extend_from_slice(&6f32.to_wasm_bytes());

        let code_section = CodeSection {
            blocks: vec![
                CodeBody {
                    locals: vec![
                        CodeLocal {
                            count: 3,
                            local_type: webassembly::F32
                        }
                    ],
                    instructions
                }
            ]
        };

        module.code_section = Some(code_section);

        // Lets build the code section. This is the actual implmentation of our add function.
        // We take 2 f32's and return 1, so our locals can just be 3 f32
        /*let locals = self.encode_local(webassembly::F32, 3);
        let instructions: Vec<u8> = Vec::new();
        let add_func = self.encode_function(vec![locals], instructions);


        let func_vec = self.encode_vector(add_func.encode());
        let code_section = self.encode_section(webassembly::SECTION_CODE, func_vec);
        module.code_section = Some(code_section);*/

        module.encode()
    }
}
