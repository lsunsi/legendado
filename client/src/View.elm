module View exposing (view)

import Browser exposing (Document)
import File
import Html exposing (Html, button, div, h1, input, li, p, text, ul)
import Html.Attributes exposing (placeholder, style, value)
import Html.Events exposing (onClick, onInput)
import Model exposing (Authentication(..), Model, SubtitleForList, Teledata(..))
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


authorizationView : Authentication -> Html Msg
authorizationView authentication =
    case authentication of
        Unrequested ->
            div []
                [ button [ onClick LoginRequestClicked ] [ text "login" ]
                ]

        RequestEmail emailInput ->
            div []
                [ input [ value emailInput, placeholder "email", onInput LoginEmailInputChanged ] []
                , button [ onClick LoginEmailSubmitClicked ] [ text "login" ]
                ]

        RequestPin email pinInput ->
            div []
                [ div [] [ text email ]
                , input [ value pinInput, placeholder "pin", onInput LoginPinInputChanged ] []
                , button [ onClick LoginPinSubmitClicked ] [ text "login" ]
                ]

        Authenticated email _ ->
            text ("Logged in as " ++ email)


view : Model -> Document Msg
view model =
    { title = "Legendado"
    , body =
        [ h1 [] [ text "Legendado" ]
        , authorizationView model.authentication
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
