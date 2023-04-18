import ffi from "ffi-napi";
import ref from "ref-napi";
import ref_struct from "ref-struct-di";
import ref_array from "ref-array-di";

const StructType = ref_struct(ref);
const ArrayType = ref_array(ref);

const TestConfig = StructType({
  gain: ref.types.double,
  speed: ref.types.double,
  cell_mg: ref.types.double,
  canvas_width: ref.types.double,
  canvas_height: ref.types.double,
  canvas_position: ref.types.int,
});
const IntArray = ArrayType(ref.types.int);
const DoubleArray = ArrayType(ref.types.double);
const ArrayDoubleList = StructType({
  ptr: DoubleArray,
  len: ref.types.int,
});

const TestStruct = StructType({
  gain: ref.types.double,
  speed: ref.types.double,
  name: ref.types.CString,
  is_show: ref.types.bool,
});
const TestStruct2 = StructType({
  num: ref.types.double,
  desc: ref.types.CString,
});
const TestArray2 = ArrayType(TestStruct2);
const ArrayTestReutrnList = StructType({
  ptr: TestArray2,
  len: ref.types.int,
});
const TestArray = ArrayType(TestStruct);
const ArrayTestList = StructType({
  ptr: TestArray,
  len: ref.types.int,
});

const TestInfo = StructType({
  config: TestConfig,
  test_struct: ArrayTestList,
});

const ReturnStructv2 = StructType({
  num: ref.types.double,
  desc: ref.types.CString,
  is_shwo: ref.types.bool,
});

const libm = ffi.Library("./target/release/wave_from_tools.dll", {
  get_f64: ["double", ["double", "double"]],
  get_string: [ref.types.CString, ["double", "double"]],
  process_test_info: ["void", [TestInfo]],
  get_test_struct: [ReturnStructv2, []],
  test_array: [ArrayTestReutrnList, [ArrayTestList]],
  rust_free_return_struct: ["void", [ArrayTestReutrnList]],
});



console.log("get_f64", libm.get_f64(1.0, 8.0));


const rustString = libm.get_string(1.0, 8.0);
console.log("get_string",rustString);


const returnStructv2 = libm.get_test_struct();
console.log("get_test_struct", returnStructv2.toObject());


const TestStructItem = new TestStruct({
  gain: 3.0,
  speed: 1.0,
  name: "This is a test",
  is_show: true,
});
const TestStructItem2 = new TestStruct({
  gain: 7.0,
  speed: 8.0,
  name: "",
  is_show: false,
});
const TestList = new TestArray([TestStructItem, TestStructItem2]);
const TestInList = new ArrayTestList({
  ptr: TestList,
  len: TestList.length,
});
const TestArrayInfo = libm.test_array(TestInList);
let tmp = TestArrayInfo.toJSON();
// You need to modify the langth of the buffer to restore it itself
tmp.ptr.length = tmp.len;
tmp = tmp.ptr.toArray();
tmp.forEach(element => {
  console.log('element', element.toObject());
});


const wavesInfo = new TestInfo({
  config: new TestConfig({
    gain: 3.0,
    speed: 1.0,
    cell_mg: 4.0,
    canvas_width: 600.2,
    canvas_height: 800.4,
    canvas_position: 3,
  }),
  test_struct: TestInList,
});
libm.process_test_info(wavesInfo);


libm.rust_free_return_struct(TestArrayInfo);