use crate::shared::Status;

pub fn categorise(code: u16) -> Status {
    match code {
        100 => Status {
            code,
            formal_message: "Continue".to_string(),
            human_message: "The server has received the request headers and the client should proceed to send the request again, but now in full.".to_string(),
        },
        101 => Status {
            code,
            formal_message: "Switching Protocols".to_string(),
            human_message: "The requester has asked the server to switch protocols and the server has agreed to do so".to_string(),
        },
        102 => Status {
            code,
            formal_message: "Processing".to_string(),
            human_message: "The server has received and is processing the request, but no response is available yet".to_string(),
        },
        103 => Status {
            code,
            formal_message: "Early Hints".to_string(),
            human_message: "Used to return some response headers before final jsontp message".to_string(),
        },
        200 => Status {
            code,
            formal_message: "OK".to_string(),
            human_message: "The request has succeeded".to_string(),
        },
        201 => Status {
            code,
            formal_message: "Created".to_string(),
            human_message: "The request has been fulfilled and has resulted in one or more new resources being created".to_string(),
        },
        202 => Status {
            code,
            formal_message: "Accepted".to_string(),
            human_message: "The request has been accepted for processing, but the processing has not been completed".to_string(),
        },
        203 => Status {
            code,
            formal_message: "Non-Authoritative Information".to_string(),
            human_message: "The server is a transforming proxy that received a 200 OK from its origin, but is returning a modified version of the origin's response".to_string(),
        },
        204 => Status {
            code,
            formal_message: "No Content".to_string(),
            human_message: "The server successfully processed the request and is not returning any content".to_string(),
        },
        205 => Status {
            code,
            formal_message: "Reset Content".to_string(),
            human_message: "The server successfully processed the request, but is not returning any content".to_string(),
        },
        206 => Status {
            code,
            formal_message: "Partial Content".to_string(),
            human_message: "The server is delivering only part of the resource due to a range header sent by the client".to_string(),
        },
        207 => Status {
            code,
            formal_message: "Multi-Status".to_string(),
            human_message: "The message body that follows is by default an XML message and can contain a number of separate response codes, depending on how many sub-requests were made".to_string(),
        },
        208 => Status {
            code,
            formal_message: "Already Reported".to_string(),
            human_message: "The members of a DAV binding have already been enumerated in a previous reply to this request, and are not being included again".to_string(),
        },
        226 => Status {
            code,
            formal_message: "IM Used".to_string(),
            human_message: "The server has fulfilled a GET request for the resource, and the response is a representation of the result of one
            or more instance-manipulations applied to the current instance".to_string(),
        },
        300 => Status {
            code,
            formal_message: "Multiple Choices".to_string(),
            human_message: "Indicates multiple options for the resource from which the client may choose".to_string(),
        },
        301 => Status {
            code,
            formal_message: "Moved Permanently".to_string(),
            human_message: "This and all future requests should be directed to the given URI".to_string(),
        },
        302 => Status {
            code,
            formal_message: "Found".to_string(),
            human_message: "Tells the client to look at (browse to) another URL".to_string(),
        },
        303 => Status {
            code,
            formal_message: "See Other".to_string(),
            human_message: "The response to the request can be found under another URI using a GET method".to_string(),
        },
        304 => Status {
            code,
            formal_message: "Not Modified".to_string(),
            human_message: "Indicates that the resource has not been modified since the version specified by the request headers If-Modified-Since or If-None-Match".to_string(),
        },
        305 => Status {
            code,
            formal_message: "Use Proxy".to_string(),
            human_message: "The requested resource is available only through a proxy, the address for which is provided in the response".to_string(),
        },
        306 => Status {
            code,
            formal_message: "Switch Proxy".to_string(),
            human_message: "No longer used".to_string(),
        },
        307 => Status {
            code,
            formal_message: "Temporary Redirect".to_string(),
            human_message: "The request should be repeated with another URI; however, future requests should still use the original URI".to_string(),
        },
        308 => Status {
            code,
            formal_message: "Permanent Redirect".to_string(),
            human_message: "The request and all future requests should be repeated using another URI".to_string(),
        },
        400 => Status {
            code,
            formal_message: "Bad Request".to_string(),
            human_message: "The server cannot or will not process the request due to an apparent client error".to_string(),
        },
        401 => Status {
            code,
            formal_message: "Unauthorized".to_string(),
            human_message: "Similar to 403 Forbidden, but specifically for use when authentication is required and has failed or has not yet been provided".to_string(),
        },
        402 => Status {
            code,
            formal_message: "Payment Required".to_string(),
            human_message: "Reserved for future use".to_string(),
        },
        403 => Status {
            code,
            formal_message: "Forbidden".to_string(),
            human_message: "The request contained valid data and was understood by the server, but the server is refusing action".to_string(),
        },
        404 => Status {
            code,
            formal_message: "Not Found".to_string(),
            human_message: "The requested resource could not be found but may be available in the future".to_string(),
        },
        405 => Status {
            code,
            formal_message: "Method Not Allowed".to_string(),
            human_message: "A request method is not supported for the requested resource".to_string(),
        },
        406 => Status {
            code,
            formal_message: "Not Acceptable".to_string(),
            human_message: "The requested resource is capable of generating only content not acceptable according to the Accept headers sent in the request".to_string(),
        },
        407 => Status {
            code,
            formal_message: "Proxy Authentication Required".to_string(),
            human_message: "The client must first authenticate itself with the proxy".to_string(),
        },
        408 => Status {
            code,
            formal_message: "Request Timeout".to_string(),
            human_message: "The server timed out waiting for the request".to_string(),
        },
        409 => Status {
            code,
            formal_message: "Conflict".to_string(),
            human_message: "Indicates that the request could not be processed because of conflict in the request".to_string(),
        },
        410 => Status {
            code,
            formal_message: "Gone".to_string(),
            human_message: "Indicates that the resource requested is no longer available and will not be available again".to_string(),
        },
        411 => Status {
            code,
            formal_message: "Length Required".to_string(),
            human_message: "The request did not specify the length of its content, which is required by the requested resource".to_string(),
        },
        412 => Status {
            code,
            formal_message: "Precondition Failed".to_string(),
            human_message: "The server does not meet one of the preconditions that the requester put on the request".to_string(),
        },
        413 => Status {
            code,
            formal_message: "Payload Too Large".to_string(),
            human_message: "The request is larger than the server is willing or able to process".to_string(),
        },
        414 => Status {
            code,
            formal_message: "URI Too Long".to_string(),
            human_message: "The URI provided was too long for the server to process".to_string(),
        },
        415 => Status {
            code,
            formal_message: "Unsupported Media Type".to_string(),
            human_message: "The request entity has a media type which the server or resource does not support".to_string(),
        },
        416 => Status {
            code,
            formal_message: "Range Not Satisfiable".to_string(),
            human_message: "The client has asked for a portion of the file, but the server cannot supply that portion".to_string(),
        },
        417 => Status {
            code,
            formal_message: "Expectation Failed".to_string(),
            human_message: "The server cannot meet the requirements of the Expect request-header field".to_string(),
        },
        418 => Status {
            code,
            formal_message: "I'm a teapot".to_string(),
            human_message: "The server refuses the attempt to brew coffee with a teapot".to_string(),
        },
        421 => Status {
            code,
            formal_message: "Misdirected Request".to_string(),
            human_message: "The request was directed at a server that is not able to produce a response".to_string(),
        },
        422 => Status {
            code,
            formal_message: "Unprocessable Entity".to_string(),
            human_message: "The request was well-formed but was unable to be followed due to semantic errors".to_string(),
        },
        423 => Status {
            code,
            formal_message: "Locked".to_string(),
            human_message: "The resource that is being accessed is locked".to_string(),
        },
        424 => Status {
            code,
            formal_message: "Failed Dependency".to_string(),
            human_message: "The request failed due to failure of a previous request".to_string(),
        },
        425 => Status {
            code,
            formal_message: "Too Early".to_string(),
            human_message: "Indicates that the server is unwilling to risk processing a request that might be replayed".to_string(),
        },
        426 => Status {
            code,
            formal_message: "Upgrade Required".to_string(),
            human_message: "The client should switch to a different protocol such as TLS/1.0".to_string(),
        },
        428 => Status {
            code,
            formal_message: "Precondition Required".to_string(),
            human_message: "The origin server requires the request to be conditional".to_string(),
        },
        429 => Status {
            code,
            formal_message: "Too Many Requests".to_string(),
            human_message: "The user has sent too many requests in a given amount of time".to_string(),
        },
        431 => Status {
            code,
            formal_message: "Request Header Fields Too Large".to_string(),
            human_message: "The server is unwilling to process the request because either an individual header field, or all the header fields collectively, are too large".to_string(),
        },
        451 => Status {
            code,
            formal_message: "Unavailable For Legal Reasons".to_string(),
            human_message: "A server operator has received a legal demand to deny access to a resource or to a set of resources that includes the requested resource".to_string(),
        },
        500 => Status {
            code,
            formal_message: "Internal Server Error".to_string(),
            human_message: "A generic error message, given when an unexpected condition was encountered and no more specific message is suitable".to_string(),
        },
        501 => Status {
            code,
            formal_message: "Not Implemented".to_string(),
            human_message: "The server either does not recognize the request method, or it lacks the ability to fulfill the request".to_string(),
        },
        502 => Status {
            code,
            formal_message: "Bad Gateway".to_string(),
            human_message: "The server was acting as a gateway or proxy and received an invalid response from the upstream server".to_string(),
        },
        503 => Status {
            code,
            formal_message: "Service Unavailable".to_string(),
            human_message: "The server is not ready to handle the request".to_string(),
        },
        504 => Status {
            code,
            formal_message: "Gateway Timeout".to_string(),
            human_message: "The server was acting as a gateway or proxy and did not receive a timely response from the upstream server".to_string(),
        },
        505 => Status {
            code,
            formal_message: "jsontp Version Not Supported".to_string(),
            human_message: "The server does not support the jsontp protocol version used in the request".to_string(),
        },
        506 => Status {
            code,
            formal_message: "Variant Also Negotiates".to_string(),
            human_message: "Transparent content negotiation for the request results in a circular reference".to_string(),
        },
        507 => Status {
            code,
            formal_message: "Insufficient Storage".to_string(),
            human_message: "The server is unable to store the representation needed to complete the request".to_string(),
        },
        508 => Status {
            code,
            formal_message: "Loop Detected".to_string(),
            human_message: "The server detected an infinite loop while processing the request".to_string(),
        },
        510 => Status {
            code,
            formal_message: "Not Extended".to_string(),
            human_message: "Further extensions to the request are required for the server to fulfill it".to_string(),
        },
        511 => Status {
            code,
            formal_message: "Network Authentication Required".to_string(),
            human_message: "The client needs to authenticate to gain network access".to_string(),
        },
        _ => Status {
            code: 500,
            formal_message: "Internal Server Error".to_string(),
            human_message: "The callback provided an incorrect error code.".to_string(),
        },
        
        
    }
}