const fs = require("fs")
const ytdl = require("ytdl-core")

const serverConfig = require("./serverConfigs/CKConfig.json")

let dispatcher;

function writeToJson() {
    fs.writeFile("./serverConfigs/CKConfig.json", JSON.stringify(serverConfig, null, 4), function writeJSON(err) {
        if (err) { return console.log(err); }
    });
}

module.exports = {
    getSongTitle: async function (link) {
        const info = await ytdl.getInfo(link);
        return info.videoDetails.title
    },
    
    play: function (message, connection) {

        if (serverConfig.trackPosition >= serverConfig.musicQueue.length && serverConfig.loopQueue) {
            serverConfig.trackPosition = 0
        }
        dispatcher = connection.play(ytdl(serverConfig.musicQueue[serverConfig.trackPosition], { filter: 'audioonly' }))

        dispatcher.on("finish", () => {
            if (!serverConfig.loopTrack) {
                serverConfig.trackPosition += 1;
            }

            console.log(serverConfig.trackPosition)
            console.log(serverConfig.musicQueue.length)
            if (serverConfig.trackPosition <= serverConfig.musicQueue.length) {
                console.log(serverConfig.musicQueue)
                writeToJson()

                module.exports.play(message, connection)
            } else {
                serverConfig.musicQueue = []
                writeToJson()
                console.log("Disconnecting")
                connection.disconnect();
            }
        })
    },

    skip: function () {
        if (dispatcher) { dispatcher.end() }
    },

    clear: function () {
        serverConfig.musicQueue = []
        writeToJson()

    }

}