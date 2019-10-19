module Update exposing (Msg(..), update)

import Api
import File exposing (File)
import File.Download as FileDownload
import File.Select as FileSelect
import Model exposing (Authentication(..), HttpResult, Model, Route(..), SubtitleForDownload, SubtitleForList, SubtitleForUpload, Teledata(..))


type Msg
    = UploadAskButtonClicked
    | UploadFileSelected File
    | UploadConfirmButtonClicked SubtitleForUpload
    | UploadResponseReceived (HttpResult ())
    | SubtitlesResponseReceived (HttpResult (List SubtitleForList))
    | SubtitleClicked Int
    | SubtitleResponseReceived (HttpResult SubtitleForDownload)
    | LoginRequestClicked
    | LoginEmailInputChanged String
    | LoginPinInputChanged String
    | LoginEmailSubmitClicked
    | LoginPinSubmitClicked
    | LoginPinRequestResponseReceived (HttpResult ())
    | LoginPinAuthenticationResponseReceived (HttpResult String)
    | NavbarBrandClicked


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
            case model.authentication of
                Authenticated _ token ->
                    ( model, Api.postSubtitleForUpload token file UploadResponseReceived )

                _ ->
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
            case model.authentication of
                Authenticated _ token ->
                    ( model, Api.getSubtitleForDownload token id SubtitleResponseReceived )

                _ ->
                    ( model, Cmd.none )

        SubtitleResponseReceived (Ok { name, mime, content }) ->
            ( model, FileDownload.bytes name mime content )

        SubtitleResponseReceived (Err sub) ->
            ( model, Cmd.none )

        LoginRequestClicked ->
            ( { model | authentication = RequestEmail "" }, Cmd.none )

        LoginEmailInputChanged email ->
            ( { model | authentication = RequestEmail email }, Cmd.none )

        LoginPinInputChanged pin ->
            case model.authentication of
                RequestPin email _ ->
                    ( { model | authentication = RequestPin email pin }, Cmd.none )

                _ ->
                    ( model, Cmd.none )

        LoginEmailSubmitClicked ->
            case model.authentication of
                RequestEmail email ->
                    ( model, Api.requestLoginPin email LoginPinRequestResponseReceived )

                _ ->
                    ( model, Cmd.none )

        LoginPinSubmitClicked ->
            case model.authentication of
                RequestPin email pin ->
                    ( model, Api.authenticateLoginPin email pin LoginPinAuthenticationResponseReceived )

                _ ->
                    ( model, Cmd.none )

        LoginPinRequestResponseReceived (Ok ()) ->
            case model.authentication of
                RequestEmail email ->
                    ( { model | authentication = RequestPin email "" }, Cmd.none )

                _ ->
                    ( model, Cmd.none )

        LoginPinRequestResponseReceived (Err _) ->
            ( model, Cmd.none )

        LoginPinAuthenticationResponseReceived (Ok token) ->
            case model.authentication of
                RequestPin email _ ->
                    ( { model | authentication = Authenticated email token }, Cmd.none )

                _ ->
                    ( model, Cmd.none )

        LoginPinAuthenticationResponseReceived (Err _) ->
            ( model, Cmd.none )

        NavbarBrandClicked ->
            ( { model | route = Homepage }, Cmd.none )
