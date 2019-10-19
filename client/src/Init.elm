module Init exposing (init)

import Api
import Model exposing (Authentication(..), Model, Route(..), Teledata(..))
import Update exposing (Msg(..))


init : () -> ( Model, Cmd Msg )
init () =
    ( { subtitleForUpload = Nothing
      , subtitles = Loading
      , authentication = Unrequested
      , route = Uploads
      }
    , Api.getSubtitlesForList SubtitlesResponseReceived
    )
