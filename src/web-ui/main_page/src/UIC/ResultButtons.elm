module UIC.ResultButtons exposing (logonButton, resultButton)

import Element exposing (..)
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Element.Input as Input
import Model exposing (Model, ResultButtonState(..), SessionState(..))
import Simple.Animation as Animation exposing (Animation)
import Simple.Animation.Animated as Animated
import Simple.Animation.Property as AnimationProperty
import Themes.Base exposing (getTheme)


logonButton : Model -> String -> Maybe msg -> Element msg
logonButton model labelText maybeMsg =
    let
        colorScheme =
            getTheme model.theme

        sessionState =
            model.session

        buttonState =
            case sessionState of
                NoSession ->
                    ResultButtonClickedOk

                SessionLogout ->
                    ResultButtonNoOp

                _ ->
                    ResultButtonClickedErr
    in
    animatedButton buttonState
        [ Border.width 1
        , Border.rounded 5
        , Border.color colorScheme.simpleButtonBorder
        , Background.color colorScheme.simpleButtonBackground
        , Font.size 14
        , paddingXY 8 0
        , centerX
        , height <| px 30
        , mouseDown
            [ Border.color colorScheme.simpleButtonBorderFocused
            , Background.color colorScheme.simpleButtonBackgroundFocused
            ]
        , mouseOver
            [ Border.color colorScheme.simpleButtonBorderMouseOver
            , Background.color colorScheme.simpleButtonBackgroundMouseOver
            ]
        , focused
            [ Border.color colorScheme.simpleButtonBorderFocused ]
        ]
        { onPress = maybeMsg
        , label =
            -- The label can be any element, so for example, the button
            -- can contain an image
            el [ centerX, centerY, Font.color colorScheme.simpleButtonFontColor ] <|
                text labelText
        }


resultButton : Model -> ResultButtonState -> String -> Maybe msg -> Element msg
resultButton model buttonState labelText maybeMsg =
    let
        colorScheme =
            getTheme model.theme
    in
    case buttonState of
        _ ->
            animatedButton buttonState
                [ Border.width 1
                , Border.rounded 5
                , Border.color colorScheme.simpleButtonBorder
                , Background.color colorScheme.simpleButtonBackground
                , Font.size 14
                , paddingXY 8 0
                , centerX
                , height <| px 30
                , mouseDown
                    [ Border.color colorScheme.simpleButtonBorderFocused
                    , Background.color colorScheme.simpleButtonBackgroundFocused
                    ]
                , mouseOver
                    [ Border.color colorScheme.simpleButtonBorderMouseOver
                    , Background.color colorScheme.simpleButtonBackgroundMouseOver
                    ]
                , focused
                    [ Border.color colorScheme.simpleButtonBorderFocused ]
                ]
                { onPress = maybeMsg
                , label =
                    -- The label can be any element, so for example, the button
                    -- can contain an image
                    el [ centerX, centerY, Font.color colorScheme.simpleButtonFontColor ] <|
                        text labelText
                }


animatedButton : ResultButtonState -> List (Attribute msg) -> { onPress : Maybe msg, label : Element msg } -> Element msg
animatedButton btnState =
    case btnState of
        ResultButtonNoOp ->
            Input.button

        ResultButtonClickedOk ->
            animatedUi Input.button okBorder

        ResultButtonClickedErr ->
            animatedUi Input.button errBorder


animatedUi : (List (Attribute msg) -> children -> Element msg) -> Animation -> List (Attribute msg) -> children -> Element msg
animatedUi =
    Animated.ui
        { behindContent = Element.behindContent
        , htmlAttribute = Element.htmlAttribute
        , html = Element.html
        }


animationDuration : Int
animationDuration =
    2000


okBorder : Animation
okBorder =
    Animation.fromTo
        { duration = animationDuration
        , options = [ Animation.loop ]
        }
        -- [ AnimationProperty.borderColor "#54A358" ] -- PRODUCTION
        [ AnimationProperty.borderColor "#00FF00" ]
        [ AnimationProperty.borderColor "#464E57" ]


errBorder : Animation
errBorder =
    Animation.fromTo
        { duration = animationDuration
        , options = [ Animation.count 2 ]
        }
        -- [ AnimationProperty.borderColor "#E0524B" ] -- PRODUCTION
        [ AnimationProperty.borderColor "#FF0000" ]
        [ AnimationProperty.borderColor "#464E57" ]
