module UIC.BasicPanel exposing (basicPanel)

import Element
    exposing
        ( centerX
        , centerY
        , column
        , el
        , fill
        , height
        , paddingXY
        , px
        , row
        , spacingXY
        , text
        , width
        )
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Model exposing (Model, Msg(..))
import Themes.Base exposing (getTheme)
import UIC.Buttons exposing (simpleButton)


basicPanel : Model -> Element.Element Msg
basicPanel model =
    let
        colorScheme =
            getTheme model.theme
    in
    column
        [ centerX
        , centerY

        -- , width <| px 300
        , Border.width 1
        , Border.rounded 5
        , Border.color colorScheme.simpleButtonBorder
        , Background.color colorScheme.basicPanelHeaderBackground

        -- , Background.color colorScheme.simpleButtonBackground
        -- , spacingXY 0 10
        -- , Element.explain Debug.todo
        ]
        [ -- Panel Header
          row
            [ width fill
            , height <| px 35

            -- , Border.width 1
            , Border.widthEach { bottom = 1, left = 0, right = 0, top = 0 }
            , Border.roundEach { bottomLeft = 0, bottomRight = 0, topLeft = 4, topRight = 4 }
            , Font.family [ Font.monospace ]

            -- , Border.rounded 5
            , Border.color colorScheme.simpleButtonBorder
            , Background.color colorScheme.basicPanelHeaderBackground
            ]
            [ row [ paddingXY 10 10 ]
                [ el [] <|
                    text "1. "
                , el
                    [ Border.widthEach { bottom = 0, left = 1, right = 0, top = 0 }
                    , Border.color colorScheme.simpleButtonBorder
                    , paddingXY 10 10
                    ]
                  <|
                    Element.none
                , el
                    []
                  <|
                    text "Current theme:"
                , el
                    [ Font.bold
                    ]
                  <|
                    text model.theme
                ]
            ]
        , column
            [ paddingXY 10 10
            , spacingXY 0 10
            , Background.color colorScheme.basicPanelContentBackground
            , Border.roundEach { bottomLeft = 4, bottomRight = 4, topLeft = 0, topRight = 0 }
            ]
            [ el [ centerX, Font.size 24 ] <| text "Your Elm App is working!"
            , simpleButton model "toggle theme" <| Just ToggleTheme
            ]
        ]
