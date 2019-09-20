module Model exposing (HttpResult, Model, SubtitleForDownload, SubtitleForList, SubtitleForUpload, Teledata(..))

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


type alias Model =
    { subtitleForUpload : Maybe SubtitleForUpload
    , subtitles : Teledata (List SubtitleForList)
    , token : Maybe String
    , emailInput : String
    }
