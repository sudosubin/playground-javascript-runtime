// console.js
const test1 = async () => {
  console.log("Echo playground-javascript-runtime ...");
  console.error("Example error message ...");
};

// fs.js
const test2 = async () => {
  const path = "./task.log";

  try {
    const data = await fs.readFile(path);
    console.log("Read file from a file", data);
  } catch (error) {
    console.error(`Failed read from file: ${path}`, error);
  }
};

// fs.js
const test3 = async () => {
  const path = "./task.log";

  await fs.writeFile(path, "Simple test logging text.");
  const data = await fs.readFile(path);
  console.log("Read from a file", path, data);

  fs.unlink(path);
  console.log("File unlinked");
};

await test1();
await test2();
await test3();
