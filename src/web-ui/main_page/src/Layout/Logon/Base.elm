module Layout.Logon.Base exposing (logonLayout)

import Element
    exposing
        ( centerX
        , centerY
        , column
        , el
        , fill
        , focused
        , height
        , padding
        , px
        , rgb255
        , row
        , spacingXY
        , text
        , width
        )
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Element.Input as Input
import Model exposing (Model, Msg(..))
import Themes.Base exposing (getTheme)
import UIC.Misc exposing (horizontalSpacer)
import UIC.ResultButtons exposing (logonButton)
import Vendor.BrickpackLogos exposing (brickpackLogo)


logonLayout : Model -> Element.Element Msg
logonLayout model =
    column
        [ width <| px 350
        , height <| px 400
        , centerX
        , spacingXY 0 20
        , padding 20

        -- , Element.explain Debug.todo
        -- , Background.color <| rgb255 0xA1 0xA1 0xA1 -- DEBUG
        ]
        [ el [ height <| px 20 ] <| Element.none
        , headerPanel model
        , logonPanel model

        -- TODO
        -- , footerPanel model
        ]


headerPanel : Model -> Element.Element msg
headerPanel model =
    let
        colorScheme =
            getTheme model.theme
    in
    column
        [ width fill
        , spacingXY 0 30
        ]
        [ el [ centerX ] <| brickpackLogo 95 75 (Just "#CDD9E5")
        , row [ centerX ]
            [ el [] <| text "Sign in to "
            , el [ Font.bold ] <| text "Brickpack"
            ]
        ]


logonPanel : Model -> Element.Element Msg
logonPanel model =
    let
        colorScheme =
            getTheme model.theme

        usernameFromModel =
            case model.logon_buffer.username of
                Just username ->
                    username

                Nothing ->
                    ""

        passwordFromModel =
            case model.logon_buffer.password of
                Just password ->
                    password

                Nothing ->
                    ""
    in
    column
        [ width fill
        , Background.color colorScheme.basicPanelContentBackground
        , Border.width 1
        , Border.rounded 5
        , Border.color colorScheme.simpleButtonBorder
        , spacingXY 0 20
        ]
        [ column
            [ width fill
            , padding 20
            , spacingXY 0 20
            ]
            [ column
                [ width fill
                , spacingXY 0 5
                ]
                [ el [ Font.size 18 ] <| text "Username"
                , Input.username
                    [ Border.width 1
                    , Border.rounded 5
                    , Border.color colorScheme.simpleButtonBorder
                    , Background.color colorScheme.simpleButtonBackground
                    , Font.size 14
                    , padding 5
                    , width fill
                    , height <| px 28
                    , centerY
                    , focused [ Border.color <| rgb255 0x1E 0x1E 0x1E ]
                    ]
                    { onChange = UserTypedUsername
                    , text = usernameFromModel
                    , placeholder = Nothing
                    , label = Input.labelHidden ""
                    }
                ]
            , column
                [ width fill
                , spacingXY 0 5
                ]
                [ row [ width fill ]
                    [ el [ Font.size 18 ] <| text "Password"
                    , horizontalSpacer

                    -- , el [ Font.size 12 ] <| text "Forgot password"
                    ]
                , Input.currentPassword
                    [ Border.width 1
                    , Border.rounded 5
                    , Border.color colorScheme.simpleButtonBorder
                    , Background.color colorScheme.simpleButtonBackground
                    , Font.size 14
                    , padding 5
                    , width fill
                    , height <| px 28
                    , centerY
                    , focused [ Border.color <| rgb255 0x1E 0x1E 0x1E ]
                    ]
                    { onChange = UserTypedPassword
                    , text = passwordFromModel
                    , placeholder = Nothing
                    , label = Input.labelHidden ""
                    , show = False
                    }
                ]

            -- resultButton model btnState "Refresh" <| Just MainPage
            -- , simpleButton model "Sign in" <| Just StartSession
            , logonButton model "Sign in" <| Just StartSession
            ]
        ]


footerPanel : Model -> Element.Element msg
footerPanel model =
    let
        colorScheme =
            getTheme model.theme
    in
    row
        [ width fill
        , height <| px 40
        , Background.color colorScheme.basicPanelContentBackground
        , Border.width 1
        , Border.rounded 5
        , Border.color colorScheme.simpleButtonBorder
        ]
        [ el [] <| text "New to Brickpack?"
        , horizontalSpacer
        , el [] <| text "Create an account."
        ]
