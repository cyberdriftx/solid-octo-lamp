const fs = require("fs");

function getFileSize(filePath) {
    try {
        const stats = fs.statSync(filePath);
        console.log(`📁 File Size: ${stats.size} bytes`);
    } catch (error) {
        console.log("❌ File not found!");
    }
}

getFileSize("example.txt");
