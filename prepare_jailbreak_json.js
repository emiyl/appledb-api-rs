const fs = require("fs");
const path = require("path");

function getAllFiles(dirPath, arrayOfFiles) {
  files = fs.readdirSync(dirPath);
  arrayOfFiles = arrayOfFiles || [];
  files.forEach(function (file) {
    if (fs.statSync(dirPath + "/" + file).isDirectory()) {
      arrayOfFiles = getAllFiles(dirPath + "/" + file, arrayOfFiles);
    } else {
      arrayOfFiles.push(path.join(dirPath, "/", file));
    }
  });
  return arrayOfFiles;
}

function requireAll(p, fileType) {
  return getAllFiles(p)
    .filter((f) => f.endsWith(fileType))
    .map((f) => require("." + path.sep + f));
}

function mkdir(p) {
  if (!fs.existsSync(p)) {
    fs.mkdirSync(p);
    return true;
  } else return false;
}

function write(p, f) {
  fs.writeFileSync(p, f);
  filesWritten++;
}

function writeJson(dirName, arr, property) {
  const output_path = "./tmp"
  mkdir(output_path);
  mkdir(path.join(output_path, dirName));
  arr.map(function (x) {
    write(
      path.join(output_path, dirName, x[property].replace("/", "%2F") + ".json"),
      JSON.stringify(x),
    );
  });
}

var jailbreakFiles = requireAll("./appledb/jailbreakFiles", ".js");
var main = {};
var filesWritten = 0;

writeJson("jailbreak", jailbreakFiles, "name");

console.log("Files Written:", filesWritten);
