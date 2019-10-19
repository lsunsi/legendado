module Init exposing (init)

import Api
import Model exposing (Authentication(..), Model, Route(..), SubtitleUpload(..), Teledata(..))
import Update exposing (Msg(..))


init : () -> ( Model, Cmd Msg )
init () =
    ( { subtitleUpload = SubtitleUploadUnrequested
      , subtitles = Loading
      , authentication = Unrequested
      , route = Homepage
      }
    , Api.getSubtitlesForList SubtitlesResponseReceived
    )
