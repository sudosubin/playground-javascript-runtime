((globalThis) => {
  globalThis.fs = {
    readFile: (path) => {
      return Deno.core.opAsync("op_read_file", path);
    },
    writeFile: (path, data) => {
      return Deno.core.opAsync("op_write_file", path, data);
    },
    unlink: (path) => {
      return Deno.core.opSync("op_unlink", path);
    },
  };
})(globalThis);
