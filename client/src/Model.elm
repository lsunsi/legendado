module Model exposing (Authentication(..), HttpResult, Model, Route(..), SubtitleForDownload, SubtitleForList, SubtitleForUpload, SubtitleUpload(..), Teledata(..))

import Bytes exposing (Bytes)
import File exposing (File)
import Http


type alias HttpResult data =
    Result Http.Error data


type Teledata data
    = Unasked
    | Loading
    | Done (HttpResult data)


type alias SubtitleForUpload =
    { name : String
    , mime : String
    , content : File
    }


type alias SubtitleForList =
    { id : Int
    , name : String
    , downloadsCount : Int
    }


type alias SubtitleForDownload =
    { name : String
    , mime : String
    , content : Bytes
    }


type Authentication
    = Unrequested
    | RequestEmail String
    | RequestPin String String
    | Authenticated String String


type Route
    = Homepage
    | Uploads


type SubtitleUpload
    = SubtitleUploadUnrequested
    | SubtitleUploadSelected SubtitleForUpload
    | SubtitleUploadLoading SubtitleForUpload
    | SubtitleUploadSuccess
    | SubtitleUploadFailure


type alias Model =
    { subtitleUpload : SubtitleUpload
    , subtitles : Teledata (List SubtitleForList)
    , authentication : Authentication
    , route : Route
    }
