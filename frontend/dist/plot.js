const base64Compile = str => WebAssembly.compile(typeof Buffer !== 'undefined' ? Buffer.from(str, 'base64') : Uint8Array.from(atob(str), b => b.charCodeAt(0)));

let dv = new DataView(new ArrayBuffer());
const dataView = mem => dv.buffer === mem.buffer ? dv : dv = new DataView(mem.buffer);

const emptyFunc = () => {};

const handleTables = [];

const instantiateCore = WebAssembly.instantiate;

const T_FLAG = 1 << 30;

function rscTableCreateOwn (table, rep) {
  if (rep === 0) throw new TypeError('Invalid rep');
  const free = table[0] & ~T_FLAG;
  if (free === 0) {
    table.push(0);
    table.push(rep | T_FLAG);
    return (table.length >> 1) - 1;
  }
  table[0] = table[free << 1];
  table[free << 1] = 0;
  table[(free << 1) + 1] = rep | T_FLAG;
  return free;
}

function rscTableRemove (table, handle) {
  const scope = table[handle << 1];
  const val = table[(handle << 1) + 1];
  const own = (val & T_FLAG) !== 0;
  const rep = val & ~T_FLAG;
  if (val === 0 || (scope & T_FLAG) !== 0) throw new TypeError('Invalid handle');
  table[handle << 1] = table[0] | T_FLAG;
  table[0] = handle | T_FLAG;
  return { rep, scope, own };
}

const symbolRscHandle = Symbol('handle');

const symbolDispose = Symbol.dispose || Symbol.for('dispose');

function toInt32(val) {
  return val >> 0;
}

const utf8Decoder = new TextDecoder();

const utf8Encoder = new TextEncoder();

let utf8EncodedLen = 0;
function utf8Encode(s, realloc, memory) {
  if (typeof s !== 'string') throw new TypeError('expected a string');
  if (s.length === 0) {
    utf8EncodedLen = 0;
    return 1;
  }
  let allocLen = 0;
  let ptr = 0;
  let writtenTotal = 0;
  while (s.length > 0) {
    ptr = realloc(ptr, allocLen, 1, allocLen += s.length * 2);
    const { read, written } = utf8Encoder.encodeInto(
    s,
    new Uint8Array(memory.buffer, ptr + writtenTotal, allocLen - writtenTotal),
    );
    writtenTotal += written;
    s = s.slice(read);
  }
  utf8EncodedLen = writtenTotal;
  return ptr;
}

let exports0;
let exports1;
let exports2;
let memory0;
let realloc0;
const handleTable0 = [T_FLAG, 0];
const finalizationRegistry0= new FinalizationRegistry((handle) => {
  const { rep } = rscTableRemove(handleTable0, handle);
  exports0['0'](rep);
});

handleTables[0] = handleTable0;
function trampoline0(handle) {
  const handleEntry = rscTableRemove(handleTable0, handle);
  if (handleEntry.own) {
    
    exports0['0'](handleEntry.rep);
  }
}
const trampoline1 = rscTableCreateOwn.bind(null, handleTable0);
function trampoline2(handle) {
  return handleTable0[(handle << 1) + 1] & ~T_FLAG;
}

class Dht11{
  constructor(arg0, arg1, arg2) {
    var val0 = arg0;
    var len0 = val0.length;
    var ptr0 = realloc0(0, 0, 4, len0 * 4);
    var src0 = new Uint8Array(val0.buffer, val0.byteOffset, len0 * 4);
    (new Uint8Array(memory0.buffer, ptr0, len0 * 4)).set(src0);
    var val1 = arg1;
    var len1 = val1.length;
    var ptr1 = realloc0(0, 0, 4, len1 * 4);
    var src1 = new Uint8Array(val1.buffer, val1.byteOffset, len1 * 4);
    (new Uint8Array(memory0.buffer, ptr1, len1 * 4)).set(src1);
    var val2 = arg2;
    var len2 = val2.length;
    var ptr2 = realloc0(0, 0, 4, len2 * 4);
    var src2 = new Uint8Array(val2.buffer, val2.byteOffset, len2 * 4);
    (new Uint8Array(memory0.buffer, ptr2, len2 * 4)).set(src2);
    const ret = exports1['local:plot/plot-functions#[constructor]dht11'](ptr0, len0, ptr1, len1, ptr2, len2);
    var handle4 = ret;
    var rsc3 = new.target === Dht11 ? this : Object.create(Dht11.prototype);
    Object.defineProperty(rsc3, symbolRscHandle, { writable: true, value: handle4});
    finalizationRegistry0.register(rsc3, handle4, rsc3);
    Object.defineProperty(rsc3, symbolDispose, { writable: true, value: function () {
      finalizationRegistry0.unregister(rsc3);
      rscTableRemove(handleTable0, handle4);
      rsc3[symbolDispose] = emptyFunc;
      rsc3[symbolRscHandle] = null;
      exports0['0'](handleTable0[(handle4 << 1) + 1] & ~T_FLAG);
    }});
    return rsc3;
  }
}

function buildDht11Data(arg0, arg1, arg2) {
  var val0 = arg0;
  var len0 = val0.length;
  var ptr0 = realloc0(0, 0, 4, len0 * 4);
  var src0 = new Uint8Array(val0.buffer, val0.byteOffset, len0 * 4);
  (new Uint8Array(memory0.buffer, ptr0, len0 * 4)).set(src0);
  var val1 = arg1;
  var len1 = val1.length;
  var ptr1 = realloc0(0, 0, 4, len1 * 4);
  var src1 = new Uint8Array(val1.buffer, val1.byteOffset, len1 * 4);
  (new Uint8Array(memory0.buffer, ptr1, len1 * 4)).set(src1);
  var val2 = arg2;
  var len2 = val2.length;
  var ptr2 = realloc0(0, 0, 4, len2 * 4);
  var src2 = new Uint8Array(val2.buffer, val2.byteOffset, len2 * 4);
  (new Uint8Array(memory0.buffer, ptr2, len2 * 4)).set(src2);
  const ret = exports1['local:plot/plot-functions#build-dht11-data'](ptr0, len0, ptr1, len1, ptr2, len2);
  var handle4 = ret;
  var rsc3 = new.target === Dht11 ? this : Object.create(Dht11.prototype);
  Object.defineProperty(rsc3, symbolRscHandle, { writable: true, value: handle4});
  finalizationRegistry0.register(rsc3, handle4, rsc3);
  Object.defineProperty(rsc3, symbolDispose, { writable: true, value: function () {
    finalizationRegistry0.unregister(rsc3);
    rscTableRemove(handleTable0, handle4);
    rsc3[symbolDispose] = emptyFunc;
    rsc3[symbolRscHandle] = null;
    exports0['0'](handleTable0[(handle4 << 1) + 1] & ~T_FLAG);
  }});
  return rsc3;
}

function plotLineChart(arg0, arg1, arg2) {
  var ptr0 = utf8Encode(arg2, realloc0, memory0);
  var len0 = utf8EncodedLen;
  exports1['local:plot/plot-functions#plot-line-chart'](toInt32(arg0), toInt32(arg1), ptr0, len0);
}

function hello(arg0) {
  var ptr0 = utf8Encode(arg0, realloc0, memory0);
  var len0 = utf8EncodedLen;
  const ret = exports1['local:plot/plot-functions#hello'](ptr0, len0);
  var ptr1 = dataView(memory0).getInt32(ret + 0, true);
  var len1 = dataView(memory0).getInt32(ret + 4, true);
  var result1 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr1, len1));
  return result1;
}

const $init = (async() => {
  const module0 = base64Compile('AGFzbQEAAAABKQZgAX8AYAF/AX9gBn9/f39/fwF/YAR/f39/AGACf38Bf2AEf39/fwF/AqoBAyFbZXhwb3J0XWxvY2FsOnBsb3QvcGxvdC1mdW5jdGlvbnMUW3Jlc291cmNlLWRyb3BdZGh0MTEAACFbZXhwb3J0XWxvY2FsOnBsb3QvcGxvdC1mdW5jdGlvbnMTW3Jlc291cmNlLW5ld11kaHQxMQABIVtleHBvcnRdbG9jYWw6cGxvdC9wbG90LWZ1bmN0aW9ucxNbcmVzb3VyY2UtcmVwXWRodDExAAEDBwYCAgMEAAUFAwEAAAfrAQcsbG9jYWw6cGxvdC9wbG90LWZ1bmN0aW9ucyNbY29uc3RydWN0b3JdZGh0MTEAAypsb2NhbDpwbG90L3Bsb3QtZnVuY3Rpb25zI2J1aWxkLWRodDExLWRhdGEABClsb2NhbDpwbG90L3Bsb3QtZnVuY3Rpb25zI3Bsb3QtbGluZS1jaGFydAAFH2xvY2FsOnBsb3QvcGxvdC1mdW5jdGlvbnMjaGVsbG8ABiVsb2NhbDpwbG90L3Bsb3QtZnVuY3Rpb25zI1tkdG9yXWRodDExAAcGbWVtb3J5AgAMY2FiaV9yZWFsbG9jAAgKGAYDAAALAwAACwMAAAsDAAALAgALAwAACwAvCXByb2R1Y2VycwEMcHJvY2Vzc2VkLWJ5AQ13aXQtY29tcG9uZW50BzAuMjAxLjA');
  const module1 = base64Compile('AGFzbQEAAAABBQFgAX8AAwIBAAQFAXABAQEHEAIBMAAACCRpbXBvcnRzAQAKCwEJACAAQQARAAALAC8JcHJvZHVjZXJzAQxwcm9jZXNzZWQtYnkBDXdpdC1jb21wb25lbnQHMC4yMDEuMABLBG5hbWUAExJ3aXQtY29tcG9uZW50OnNoaW0BLwEALGR0b3ItW2V4cG9ydF1sb2NhbDpwbG90L3Bsb3QtZnVuY3Rpb25zLWRodDEx');
  const module2 = base64Compile('AGFzbQEAAAABBQFgAX8AAhUCAAEwAAAACCRpbXBvcnRzAXABAQEJBwEAQQALAQAALwlwcm9kdWNlcnMBDHByb2Nlc3NlZC1ieQENd2l0LWNvbXBvbmVudAcwLjIwMS4wABwEbmFtZQAVFHdpdC1jb21wb25lbnQ6Zml4dXBz');
  ({ exports: exports0 } = await instantiateCore(await module1));
  ({ exports: exports1 } = await instantiateCore(await module0, {
    '[export]local:plot/plot-functions': {
      '[resource-drop]dht11': trampoline0,
      '[resource-new]dht11': trampoline1,
      '[resource-rep]dht11': trampoline2,
    },
  }));
  ({ exports: exports2 } = await instantiateCore(await module2, {
    '': {
      $imports: exports0.$imports,
      '0': exports1['local:plot/plot-functions#[dtor]dht11'],
    },
  }));
  memory0 = exports1.memory;
  realloc0 = exports1.cabi_realloc;
})();

await $init;
const plotFunctions = {
  Dht11: Dht11,
  buildDht11Data: buildDht11Data,
  hello: hello,
  plotLineChart: plotLineChart,
  
};

export { plotFunctions, plotFunctions as 'local:plot/plot-functions',  }