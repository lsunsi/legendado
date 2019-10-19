module View exposing (view)

import Browser exposing (Document)
import File
import Html exposing (Html, a, button, div, form, h1, h2, header, i, input, label, li, p, section, small, span, text, u, ul)
import Html.Attributes exposing (class, classList, href, placeholder, property, style, target, title, type_, value)
import Html.Events exposing (onClick, onInput, onSubmit)
import Model exposing (Authentication(..), Model, Route(..), SubtitleForList, SubtitleUpload(..), Teledata(..))
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


navbarView : Authentication -> Html Msg
navbarView auth =
    header [ class "navbar container", style "height" "36px" ]
        [ section [ class "navbar-section" ]
            [ a [ class "navbar-brand c-hand", onClick NavbarBrandClicked ] [ text "Legendado" ]
            ]
        , section [ class "navbar-section" ]
            (case auth of
                Unrequested ->
                    [ a [ class "btn btn-link", onClick LoginRequestClicked ] [ text "entrar" ] ]

                Authenticated email _ ->
                    let
                        name =
                            email |> String.split "@" |> List.head |> Maybe.withDefault "amigo"
                    in
                    [ a [ onClick UploadsLinkClicked, class "btn btn-link cursor-hand" ] [ text "Uploads" ]
                    , span [ class "ml-2" ] [ text ("Olá, " ++ name ++ "!") ]
                    ]

                _ ->
                    [ div [] [ text "~ autenticando ~" ] ]
            )
        ]


smallModalView : List (Html Msg) -> List (Html Msg) -> Html Msg
smallModalView headerChildren bodyChildren =
    div [ class "modal modal-sm active" ]
        [ div [ class "modal-overlay" ] []
        , div [ class "modal-container" ]
            [ div [ class "modal-header" ] headerChildren
            , div [ class "modal-body" ] bodyChildren
            ]
        ]


stepsView : List String -> String -> Html Msg
stepsView steps activeStep =
    let
        stepView name =
            li
                [ classList [ ( "step-item", True ), ( "active", name == activeStep ) ] ]
                [ a [] [ text name ] ]
    in
    ul [ class "step" ] (List.map stepView steps)


authModalView : Authentication -> List (Html Msg)
authModalView auth =
    let
        authSteps =
            stepsView [ "email", "pin", "sucesso" ]
    in
    case auth of
        RequestEmail emailInput ->
            [ smallModalView [ authSteps "email" ]
                [ form [ class "form-horizontal", onSubmit LoginEmailSubmitClicked ]
                    [ div [ class "form-group" ]
                        [ label [] [ text "Qual o seu email?" ]
                        , input [ class "form-input", type_ "text", placeholder "email", onInput LoginEmailInputChanged ] []
                        ]
                    , div [ class "text-right" ]
                        [ button [ type_ "submit", class "btn btn-sm" ] [ text "me envie o pin" ]
                        ]
                    ]
                ]
            ]

        RequestPin email pinInput ->
            [ smallModalView [ authSteps "pin" ]
                [ div [] [ text "Pin enviado para o email!" ]
                , form [ class "form-horizontal", onSubmit LoginPinSubmitClicked ]
                    [ div [ class "form-group" ]
                        [ label [] [ text "Qual foi o pin enviado?" ]
                        , input [ class "form-input", type_ "text", placeholder "pin", onInput LoginPinInputChanged ] []
                        ]
                    , div [ class "text-right" ] [ button [ class "btn btn-sm" ] [ text "pode conferir" ] ]
                    ]
                ]
            ]

        _ ->
            []


heroView : Html Msg
heroView =
    section [ class "hero hero-lg" ]
        [ div [ class "hero-body text-center" ]
            [ h1 [] [ text "Legendado" ]
            , h2 [] [ text "so many srt's" ]
            ]
        ]


searchView : Html Msg
searchView =
    section []
        [ div [ class "has-icon-left p-centered", style "max-width" "600px" ]
            [ input
                [ class "form-input input-lg"
                , placeholder "Example: Breaking Bad S05E12 HDTV XviD-AFG"
                , type_ "text"
                ]
                []
            , i [ class "form-icon icon icon-arrow-right" ] []
            ]
        ]


homepageView : List (Html Msg)
homepageView =
    [ heroView
    , searchView
    ]


uploadFormView : Model -> Html Msg
uploadFormView { subtitleUpload } =
    div [ class "panel" ]
        (case subtitleUpload of
            SubtitleUploadUnrequested ->
                [ div [ class "panel-header" ]
                    [ div [ class "panel-title text-center" ] [ text "Quer subir uma legenda?" ]
                    ]
                , div [ class "panel-footer" ]
                    [ button [ class "btn btn-primary btn-block", onClick UploadAskButtonClicked ]
                        [ i [ class "icon icon-upload" ] [] ]
                    ]
                ]

            SubtitleUploadSelected file ->
                [ div [ class "panel-header" ]
                    [ div [ class "panel-title text-center text-ellipsis", title file.name ] [ text file.name ]
                    ]
                , div [ class "panel-footer" ]
                    [ button [ class "btn btn-success btn-block", onClick (UploadConfirmButtonClicked file) ]
                        [ i [ class "icon icon-upload" ] [] ]
                    ]
                ]

            SubtitleUploadLoading file ->
                [ div [ class "panel-header" ]
                    [ div [ class "panel-title text-center text-ellipsis", title file.name ] [ text file.name ]
                    ]
                , div [ class "panel-footer" ]
                    [ button [ class "btn btn-success btn-block loading" ]
                        [ i [ class "icon icon-upload" ] [] ]
                    ]
                ]

            SubtitleUploadSuccess ->
                [ div [ class "panel-header" ]
                    [ div [ class "panel-title text-center" ] [ text "Upload rolou demais!" ]
                    ]
                , div [ class "panel-footer" ]
                    [ div [ class "btn-group btn-group-block" ]
                        [ button [ class "btn mr-2 disabled" ] [ text "compartilhar" ]
                        , button [ class "btn btn-primary", onClick UploadRestartButtonClicked ] [ text "mais uma" ]
                        ]
                    ]
                ]

            SubtitleUploadFailure ->
                [ div [ class "panel-header" ]
                    [ div [ class "panel-title text-center" ] [ text "Algo de errado não deu certo" ]
                    ]
                , div [ class "panel-footer" ]
                    [ div [ class "btn-group btn-group-block" ]
                        [ a [ class "btn mr-2", href "mailto:sunsi.lucas@gmail.com" ] [ text "me xingar..." ]
                        , button [ class "btn btn-primary", onClick UploadRestartButtonClicked ] [ text "de novo" ]
                        ]
                    ]
                ]
        )


uploadsView : Model -> List (Html Msg)
uploadsView model =
    [ section [ class "hero hero-sm" ]
        [ div [ class "hero-body text-center" ]
            [ h1 [] [ text "Uploads" ]
            ]
        ]
    , div [ class "columns", style "justify-content" "center" ]
        [ div [ class "column col-2" ] [ uploadFormView model ]
        ]
    ]


view : Model -> Document Msg
view model =
    let
        routeView =
            case model.route of
                Homepage ->
                    homepageView

                Uploads ->
                    uploadsView model
    in
    { title = "Legendado"
    , body =
        authModalView model.authentication
            ++ [ navbarView model.authentication ]
            ++ routeView
    }
