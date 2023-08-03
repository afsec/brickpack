module Layout.Pages.Applets.EditorPanel exposing (appletsEditorPanel)

import Element
    exposing
        ( Element
        , centerX
        , centerY
        , clip
        , column
        , el
        , fill
        , focused
        , height
        , padding
        , paddingXY
        , px
        , rgb255
        , row
        , scrollbars
        , spacing
        , spacingXY
        , text
        , width
        )
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Element.Input as Input
import Model exposing (AppletArtifactStatus(..), AppletEntry, Model, Msg(..))
import Themes.Base exposing (getTheme)
import UIC.Buttons exposing (editableLabel, simpleButton, toolbarButton)
import UIC.Misc exposing (horizontalSpacer)
import Vendor.Helpers exposing (elementAtrributeNone)
import Vendor.MonoIconsSvg exposing (MonoIconName(..))



-- import Vendor.Monoicons.Png exposing (monoIconsPng)


appletsEditorPanel : Model -> Element.Element Msg
appletsEditorPanel model =
    let
        colorScheme =
            getTheme model.theme
    in
    column
        [ -- centerX
          centerY
        , width fill
        , height fill

        -- , height <| px 480
        , Border.width 1
        , Border.rounded 5
        , Border.color colorScheme.simpleButtonBorder
        , Background.color colorScheme.basicPanelContentBackground

        -- , Element.explain Debug.todo
        ]
        [ panelHeader model
        , panelContent model
        ]


panelHeader : Model -> Element Msg
panelHeader model =
    let
        colorScheme =
            getTheme model.theme

        pages =
            model.pages

        applets =
            pages.applets

        urlPrefix =
            "/api/runner/"

        maybeAppletArtifact =
            case applets.artifact of
                DataAppletArtifactRequestSuccess appletArtifact ->
                    Just appletArtifact

                _ ->
                    Nothing
    in
    row
        [ width fill
        , height <| px 35
        , Font.family [ Font.monospace ]
        , Border.color colorScheme.simpleButtonBorder
        , case maybeAppletArtifact of
            Just _ ->
                Border.widthEach { bottom = 1, left = 0, right = 0, top = 0 }

            Nothing ->
                elementAtrributeNone
        ]
        [ case maybeAppletArtifact of
            Just appletArtifact ->
                row [ width fill, spacingXY 10 0, paddingXY 10 0 ]
                    [ case applets.filename_buffer of
                        Just _ ->
                            toolbarButton model Nothing MonoIconClose <|
                                Just EditorRenameAppletCancel

                        Nothing ->
                            el [ width <| px 10, height <| px 20 ] <| Element.none
                    , case applets.filename_buffer of
                        Just filename ->
                            Input.text
                                [ Border.width 1
                                , Border.rounded 5
                                , Border.color colorScheme.simpleButtonBorder
                                , Background.color colorScheme.simpleButtonBackground
                                , Font.size 14
                                , padding 5
                                , centerX
                                , centerY
                                , width <| px 150
                                , height <| px 28

                                --     paddingXY 10 0
                                , focused [ Border.color <| rgb255 0x1E 0x1E 0x1E ]
                                ]
                                { label = Input.labelHidden ""
                                , onChange = EditorRenameApplet
                                , placeholder = Nothing
                                , text = filename
                                }

                        Nothing ->
                            el
                                [ width <| px 150
                                , height <| px 28
                                ]
                            <|
                                editableLabel model appletArtifact.filename <|
                                    Just (EditorRenameApplet appletArtifact.filename)
                    , case applets.filename_buffer of
                        Just _ ->
                            toolbarButton model Nothing MonoIconSave <| Just EditorRenameAppletSend

                        Nothing ->
                            Element.none
                    , simpleButton model "Run" <|
                        Just <|
                            OpenNewBrowserTab <|
                                String.concat
                                    [ urlPrefix
                                    , appletArtifact.oid
                                    ]
                    , simpleButton model "Save" <|
                        Just EditorSaveApplet
                    , simpleButton model "Close" <| Just EditorCloseApplet
                    , horizontalSpacer
                    , toolbarButton model (Just "#E0524B") MonoIconDelete <|
                        Just (EditorDeleteApplet appletArtifact.oid)
                    ]

            Nothing ->
                Element.none
        ]


panelContent : Model -> Element Msg
panelContent model =
    let
        colorScheme =
            getTheme model.theme

        pages =
            model.pages

        applets =
            pages.applets
    in
    el
        [ width fill
        , height fill
        , Border.roundEach { bottomLeft = 4, bottomRight = 4, topLeft = 0, topRight = 0 }
        , case applets.artifact of
            DataAppletArtifactRequestSuccess data ->
                Background.color <| rgb255 0x1E 0x1E 0x1E

            _ ->
                Background.color colorScheme.basicPanelContentBackground

        -- , Element.explain Debug.todo
        ]
    <|
        case applets.selected of
            Just appletEntry ->
                editorFrame model appletEntry

            Nothing ->
                Element.none


editorFrame : Model -> AppletEntry -> Element Msg
editorFrame model appletEntry =
    let
        colorScheme =
            getTheme model.theme

        pages =
            model.pages

        applets =
            pages.applets
    in
    el
        [ width fill
        , height fill
        , Font.family [ Font.monospace ]
        ]
    <|
        case applets.artifact of
            DataAppletArtifactRequestSuccess _ ->
                case applets.code_buffer of
                    Just codeBuffer ->
                        el
                            [ Border.width 0
                            , width fill
                            , height <| px 463
                            , scrollbars
                            , Font.family [ Font.monospace ]
                            , Font.size 18
                            ]
                        <|
                            Input.multiline
                                [ width fill
                                , height fill
                                , Border.width 1
                                , Border.rounded 5
                                , Border.color <| rgb255 0x1E 0x1E 0x1E
                                , Background.color <| rgb255 0x1E 0x1E 0x1E
                                , padding 0
                                , spacing 0
                                , clip
                                , focused [ Border.color <| rgb255 0x1E 0x1E 0x1E ]
                                ]
                                { label = Input.labelHidden ""
                                , onChange = UpdateCodeBuffer
                                , placeholder = Nothing
                                , spellcheck = False
                                , text = codeBuffer
                                }

                    Nothing ->
                        el
                            [ width <| px 635
                            , height <| px 443
                            ]
                        <|
                            el
                                [ width fill
                                , height fill
                                , centerX
                                , centerY
                                , Font.size 24
                                , Background.color <| rgb255 0xFF 0x00 0x00
                                ]
                            <|
                                text "Error on parsing code!"

            DataAppletArtifactRequestFailure ->
                el
                    [ width <| px 635
                    , height <| px 443
                    ]
                <|
                    el
                        [ centerX
                        , centerY
                        , Font.size 24
                        , Background.color <| rgb255 0xFF 0x2D 0x00
                        ]
                    <|
                        text "Error on load code!"

            _ ->
                Element.none
