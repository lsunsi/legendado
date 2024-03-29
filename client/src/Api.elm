module Api exposing (authenticateLoginPin, getSubtitleForDownload, getSubtitlesForList, postSubtitleForUpload, requestLoginPin)

import Bytes exposing (Bytes)
import Bytes.Encode as BytesEnc
import File
import Http
import Json.Decode as Dec exposing (Decoder)
import Json.Encode as Enc
import Model exposing (HttpResult, SubtitleForDownload, SubtitleForList, SubtitleForUpload)
import Url.Builder as Url


endpoint : String
endpoint =
    "http://localhost:8000"


authorizationHeader : String -> Http.Header
authorizationHeader token =
    Http.header "Authorization" ("Bearer " ++ token)


postSubtitleForUpload : String -> SubtitleForUpload -> (HttpResult () -> msg) -> Cmd msg
postSubtitleForUpload token { name, mime, content } msg =
    let
        url =
            Url.relative [ endpoint, "upload" ]
                [ Url.string "name" name
                , Url.string "mime" mime
                ]
    in
    Http.request
        { method = "POST"
        , headers = [ authorizationHeader token ]
        , url = url
        , body = Http.fileBody content
        , expect = Http.expectWhatever msg
        , timeout = Nothing
        , tracker = Nothing
        }


subtitleForListDecoder : Decoder SubtitleForList
subtitleForListDecoder =
    Dec.map3 SubtitleForList
        (Dec.field "id" Dec.int)
        (Dec.field "name" Dec.string)
        (Dec.field "downloads_count" Dec.int)


getSubtitlesForList : (HttpResult (List SubtitleForList) -> msg) -> Cmd msg
getSubtitlesForList msg =
    Http.get
        { url = Url.relative [ endpoint, "subtitles" ] []
        , expect = Http.expectJson msg (Dec.list subtitleForListDecoder)
        }


fileBytesDecoder : Decoder Bytes
fileBytesDecoder =
    Dec.map
        (List.map BytesEnc.unsignedInt8
            >> BytesEnc.sequence
            >> BytesEnc.encode
        )
        (Dec.list Dec.int)


subtitleForTransferDecoder : Decoder SubtitleForDownload
subtitleForTransferDecoder =
    Dec.map3 SubtitleForDownload
        (Dec.field "name" Dec.string)
        (Dec.field "mime" Dec.string)
        (Dec.field "content" fileBytesDecoder)


getSubtitleForDownload : String -> Int -> (HttpResult SubtitleForDownload -> msg) -> Cmd msg
getSubtitleForDownload token id msg =
    Http.request
        { method = "GET"
        , headers = [ authorizationHeader token ]
        , url = Url.relative [ endpoint, "subtitles", String.fromInt id ] []
        , body = Http.emptyBody
        , expect = Http.expectJson msg subtitleForTransferDecoder
        , timeout = Nothing
        , tracker = Nothing
        }


requestLoginPin : String -> (HttpResult () -> msg) -> Cmd msg
requestLoginPin email msg =
    Http.post
        { url = Url.relative [ endpoint, "pin", "request" ] []
        , expect = Http.expectWhatever msg
        , body = Http.jsonBody (Enc.string email)
        }


authenticateLoginPin : String -> String -> (HttpResult String -> msg) -> Cmd msg
authenticateLoginPin email pin msg =
    Http.post
        { url = Url.relative [ endpoint, "pin", "authenticate" ] []
        , expect = Http.expectString msg
        , body =
            Http.jsonBody
                (Enc.object
                    [ ( "email", Enc.string email )
                    , ( "pin", Enc.string pin )
                    ]
                )
        }
