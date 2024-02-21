import * as net from 'net';



const handleConnection = conn => {
    const onConnData = (data: string) => {
        /*
        {
            "jsontp": "1.0", // required and in the format `major.minor(-rcx)` (explained more in the `jsontp` versioning section)
            "type": "response",
            "status": {
                "code": 200, // a valid `HTTP` status code (not 1xx)
                "formal-message": "OK", // a valid `HTTP` status message
                "human-message": "The request was successful." // a human-readable message, the contents of which is up to the discretion of the server, but should be helpful to the client. It is required, but can be a copy of the `formal-message` if the server wishes.
            },
            "resource": "/path/to/resource",
            "headers": {
                "date": "2024-01-01T00:00:00Z+00:00", // required and always in this format. Should always be in UTC, but a compliant client should accept any timezone
                "language": "en-US", // required and always in this format (explained more in the `response` headers section)
            },
            "body": {
                "key1": "value1", // optional

                "content": "raw text to be sent", // must always be present, even if empty

                "encoding": "gzip" // required, but can be `identity` if no encoding is used
            }
        }
        */
        console.log('connection data from %s: %j', remoteAddress, data);
        // conn.write(data.toUpperCase());
        try {
            try {
                var request = JSON.parse(data);
                console.log(request);
                if (request.type === undefined) {
                    throw new Error("A request type is required.");
                }
                if (request.jsontp === undefined) {
                    throw new Error("jsontp attribute with a proper version is required.")
                }
                if (request.body.content === undefined) {
                    throw new Error("A body content is required.");
                }
                if (request.body.encoding === undefined) {
                    throw new Error("A body encoding is required. Use 'identity' if unsure.");
                }
            }
            catch (err) {
                throw new Error('JSON data could not be parsed');
            }
        }
        catch (err) {
            console.log(`JSON data could not be parsed: ${remoteAddress} ${data} ${err}`);
            var errMsg = {
                jsontp: "1.0-rc1",
                type: "response",
                status: {
                    code: 400,
                    "formal-message": "Bad Request",
                    "human-message": err.message
                },
                resource: "/",
                headers: {
                    date: new Date().toISOString(),
                    language: "en-US",
                    "content-type": "text/plain"
                },
                body: {
                    content: err.message,
                    encoding: "identity"
                }
            }
            conn.write(JSON.stringify(errMsg));
        }
    }
    const onConnClose = () => {
        console.log('connection from %s closed', remoteAddress);
    }
    const onConnError = (err: {message: string}) => {
        console.log('Connection %s error: %s', remoteAddress, err.message);
    }

    var remoteAddress = conn.remoteAddress + ':' + conn.remotePort; 
    // console.log(conn);
    conn.setEncoding('utf8');
    conn.on('data', onConnData);
    conn.once('close', onConnClose);
    conn.on('error', onConnError);
}
var server = net.createServer();
server.on('connection', handleConnection);

server.listen(8080, () => {
    console.log('server listening on port %j', server.address());
});