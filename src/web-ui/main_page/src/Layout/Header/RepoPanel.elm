module Layout.Header.RepoPanel exposing (repoPanel)

import Element
    exposing
        ( centerX
        , centerY
        , column
        , el
        , fill
        , focused
        , height
        , mouseDown
        , mouseOver
        , paddingXY
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
import Model exposing (Model, Msg(..), Page(..))
import Themes.Base exposing (getTheme)


repoPanel : Model -> Page -> String -> Maybe Int -> Element.Element Msg
repoPanel model page repoLabel maybeRepoCount =
    let
        colorScheme =
            getTheme model.theme

        pages =
            model.pages
    in
    --
    Input.button
        [ centerY
        , centerX
        , Font.size 14
        , Border.width 1
        , Border.rounded 5
        , Background.color colorScheme.basicPanelContentBackground
        , mouseDown [ Border.color <| rgb255 0x54 0xA3 0x58 ] -- GREEN
        , mouseOver [ Border.color <| rgb255 0x53 0x9B 0xF5 ] -- BLUE
        , if pages.current == page then
            -- GREEN
            Border.color <| rgb255 0x54 0xA3 0x58

          else
            Border.color colorScheme.simpleButtonBorder
        , focused
            -- GREEN
            [ Border.color <| rgb255 0x54 0xA3 0x58 ]
        ]
        { onPress = Just <| ShowPage page
        , label =
            -- The label can be any element, so for example, the button
            -- can contain an image
            column
                [ centerX
                , centerY
                ]
                [ -- Panel Header
                  row
                    [ width fill
                    , height <| px 35
                    , Border.widthEach { bottom = 1, left = 0, right = 0, top = 0 }
                    , Border.roundEach { bottomLeft = 0, bottomRight = 0, topLeft = 4, topRight = 4 }
                    , Font.family [ Font.monospace ]
                    , Border.color colorScheme.simpleButtonBorder
                    , Background.color colorScheme.basicPanelHeaderBackground
                    ]
                    [ row
                        [ width fill
                        , paddingXY 10 10
                        ]
                        [ el [ centerX ] <|
                            text repoLabel
                        ]
                    ]

                -- Panel Content
                , column
                    [ paddingXY 10 10
                    , spacingXY 0 10
                    , Background.color colorScheme.basicPanelContentBackground
                    , Border.roundEach { bottomLeft = 4, bottomRight = 4, topLeft = 0, topRight = 0 }
                    , width fill
                    ]
                    [ el [ centerX, Font.size 24 ] <|
                        text <|
                            case maybeRepoCount of
                                Just repoCount ->
                                    String.fromInt repoCount

                                Nothing ->
                                    " "
                    ]
                ]
        }
