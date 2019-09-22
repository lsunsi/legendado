module Init exposing (init)

import Api
import Model exposing (Model, Teledata(..))
import Update exposing (Msg)


init : () -> ( Model, Cmd Msg )
init () =
    ( { subtitleForUpload = Nothing
      , subtitles = Unasked
      , token = Nothing
      , emailInput = ""
      }
    , Cmd.none
    )
