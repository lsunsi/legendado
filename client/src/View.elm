module View exposing (view)

import Browser exposing (Document)
import File
import Html exposing (Html, button, div, h1, input, li, p, text, ul)
import Html.Attributes exposing (placeholder, style)
import Html.Events exposing (onClick, onInput)
import Model exposing (Model, SubtitleForList, Teledata(..))
import Update exposing (Msg(..))


subtitlesListView : List SubtitleForList -> Html Msg
subtitlesListView list =
    let
        itemView { id, name, downloadsCount } =
            li
                [ onClick (SubtitleClicked id), style "padding" "8px", style "cursor" "pointer" ]
                [ text (name ++ " (" ++ String.fromInt downloadsCount ++ ")") ]
    in
    ul [] (List.map itemView list)


view : Model -> Document Msg
view model =
    { title = "Legendado"
    , body =
        [ h1 [] [ text "Legendado" ]
        , case model.token of
            Just _ ->
                div [] [ text "logged in" ]

            Nothing ->
                div []
                    [ input [ placeholder "email", onInput LoginInputChanged ] []
                    , button [ onClick LoginButtonClicked ] [ text "login" ]
                    ]
        , case model.subtitleForUpload of
            Just file ->
                div []
                    [ p [] [ text file.name ]
                    , p [] [ text file.mime ]
                    , button [ onClick (UploadConfirmButtonClicked file) ] [ text "confirm upload" ]
                    ]

            Nothing ->
                button [ onClick UploadAskButtonClicked ] [ text "upload a subtitle" ]
        , case model.subtitles of
            Done (Ok subs) ->
                subtitlesListView subs

            _ ->
                div [] [ text ":eyes:" ]
        ]
    }
