module Main exposing (..)

import Browser
import Init
import Model
import Update
import View


main : Program () Model.Model Update.Msg
main =
    Browser.document
        { view = View.view
        , init = Init.init
        , update = Update.update
        , subscriptions = always Sub.none
        }
