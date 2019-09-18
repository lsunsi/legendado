module Update exposing (Msg(..), update)

import Api
import File exposing (File)
import File.Download as FileDownload
import File.Select as FileSelect
import Model exposing (HttpResult, Model, SubtitleForDownload, SubtitleForList, SubtitleForUpload, Teledata(..))


type Msg
    = UploadAskButtonClicked
    | UploadFileSelected File
    | UploadConfirmButtonClicked SubtitleForUpload
    | UploadResponseReceived (HttpResult ())
    | SubtitlesResponseReceived (HttpResult (List SubtitleForList))
    | SubtitleClicked Int
    | SubtitleResponseReceived (HttpResult SubtitleForDownload)
    | LoginInputChanged String
    | LoginButtonClicked
    | LoginResponseReceived (HttpResult String)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UploadAskButtonClicked ->
            ( model, FileSelect.file [] UploadFileSelected )

        UploadFileSelected file ->
            let
                subtitle =
                    SubtitleForUpload (File.name file) (File.mime file) file
            in
            ( { model | subtitleForUpload = Just subtitle }, Cmd.none )

        UploadConfirmButtonClicked file ->
            case model.token of
                Just token ->
                    ( model, Api.postSubtitleForUpload token file UploadResponseReceived )

                Nothing ->
                    ( model, Cmd.none )

        UploadResponseReceived (Ok _) ->
            ( { model | subtitleForUpload = Nothing, subtitles = Loading }
            , Api.getSubtitlesForList SubtitlesResponseReceived
            )

        UploadResponseReceived (Err _) ->
            ( model, Cmd.none )

        SubtitlesResponseReceived res ->
            ( { model | subtitles = Done res }, Cmd.none )

        SubtitleClicked id ->
            ( model, Api.getSubtitleForDownload id SubtitleResponseReceived )

        SubtitleResponseReceived (Ok { name, mime, content }) ->
            ( model, FileDownload.bytes name mime content )

        SubtitleResponseReceived (Err sub) ->
            ( model, Cmd.none )

        LoginInputChanged email ->
            ( { model | emailInput = email }, Cmd.none )

        LoginButtonClicked ->
            ( model, Api.login model.emailInput LoginResponseReceived )

        LoginResponseReceived (Ok token) ->
            ( { model | token = Just token }, Cmd.none )

        LoginResponseReceived (Err _) ->
            ( model, Cmd.none )
