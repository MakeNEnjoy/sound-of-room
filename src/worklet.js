registerProcessor("WasmProcessor", class WasmProcessor extends AudioWorkletProcessor {
    constructor(options) {
        super();
        let [module, memory, handle] = options.processorOptions;
        bindgen.initSync(module, memory);
        this.processor = bindgen.WasmAudioProcessor.unpack(handle);
    }
    process(inputs, outputs) {
        return this.processor.process(inputs[0][0], outputs[0][0]);
    }
});
