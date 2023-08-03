module UIC.Buttons exposing
    (  editableLabel
       -- , resultButton

    , simpleButton
    ,  toolbarButton
       -- , toolbarResultButton

    )

import Element
    exposing
        ( Element
        , centerX
        , centerY
        , el
        , focused
        , height
        , mouseDown
        , mouseOver
        , paddingEach
        , paddingXY
        , px
        , rgb255
        , text
        , width
        )
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Element.Input as Input
import Model exposing (Model)
import Themes.Base exposing (getTheme)
import Vendor.MonoIconsSvg exposing (MonoIconName, monoIconsSvg)


simpleButton : Model -> String -> Maybe msg -> Element msg
simpleButton model labelText maybeMsg =
    let
        colorScheme =
            getTheme model.theme
    in
    Input.button
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



-- resultButton : Model -> String -> Maybe msg -> Element msg
-- resultButton model labelText maybeMsg =
--     let
--         colorScheme =
--             getTheme model.theme
--     in
--     Input.button
--         [ Border.width 1
--         , Border.rounded 5
--         , Border.color colorScheme.simpleButtonBorder
--         , Background.color colorScheme.simpleButtonBackground
--         , Font.size 14
--         , paddingXY 8 0
--         , centerX
--         , height <| px 30
--         , mouseDown
--             [ Border.color colorScheme.simpleButtonBorderFocused
--             , Background.color colorScheme.simpleButtonBackgroundFocused
--             ]
--         , mouseOver
--             [ Border.color colorScheme.simpleButtonBorderMouseOver
--             , Background.color colorScheme.simpleButtonBackgroundMouseOver
--             ]
--         , focused
--             [ Border.color colorScheme.simpleButtonBorderFocused ]
--         ]
--         { onPress = maybeMsg
--         , label =
--             -- The label can be any element, so for example, the button
--             -- can contain an image
--             el [ centerX, centerY, Font.color colorScheme.simpleButtonFontColor ] <|
--                 text labelText
--         }
-- toolbarResultButton : Model -> MonoIconName -> Maybe msg -> Element msg
-- toolbarResultButton model iconName maybeMsg =
--     let
--         colorScheme =
--             getTheme model.theme
--     in
--     -- TODO: Save button could have an animation from GREEN to normal color when
--     -- TODO:  button is clicked and `save applet` process is successfuly done and
--     -- TODO:  RED to normal color when the occurs an error.
--     Input.button
--         [ Border.width 1
--         , Border.rounded 5
--         , Border.color colorScheme.simpleButtonBorder
--         , Background.color colorScheme.simpleButtonBackground
--         , centerX
--         , centerY
--         , width <| px 28
--         , height <| px 28
--         , mouseDown
--             [ Border.color colorScheme.simpleButtonBorderFocused
--             , Background.color colorScheme.simpleButtonBackgroundFocused
--             ]
--         , mouseOver
--             [ Border.color colorScheme.simpleButtonBorderMouseOver
--             , Background.color colorScheme.simpleButtonBackgroundMouseOver
--             ]
--         , focused
--             -- [ Border.color colorScheme.simpleButtonBorderFocused ]
--             [ Border.color <| rgb255 0x54 0xA3 0x58 ]
--         ]
--         { onPress = maybeMsg
--         , label =
--             -- The label can be any element, so for example, the button
--             -- can contain an image
--             el
--                 [ centerX
--                 , centerY
--                 , width <| px 24
--                 , height <| px 24
--                 , paddingEach { top = 2, right = 0, bottom = 0, left = 2 }
--                 ]
--             <|
--                 monoIconsSvg iconName 20 20 Nothing
--         }


toolbarButton : Model -> Maybe String -> MonoIconName -> Maybe msg -> Element msg
toolbarButton model maybeStrcolor iconName maybeMsg =
    let
        colorScheme =
            getTheme model.theme
    in
    Input.button
        [ Border.width 1
        , Border.rounded 5
        , Border.color colorScheme.simpleButtonBorder
        , Background.color colorScheme.simpleButtonBackground

        -- , Font.size 14
        -- , paddingXY 8 0
        , centerX
        , centerY
        , width <| px 28
        , height <| px 28
        , mouseDown
            [ Border.color colorScheme.simpleButtonBorderFocused
            , Background.color colorScheme.simpleButtonBackgroundFocused
            ]
        , mouseOver
            [ Border.color colorScheme.simpleButtonBorderMouseOver
            , Background.color colorScheme.simpleButtonBackgroundMouseOver
            ]
        , focused
            -- [ Border.color colorScheme.simpleButtonBorderFocused ]
            [ Border.color <| rgb255 0x54 0xA3 0x58 ]
        ]
        { onPress = maybeMsg
        , label =
            -- The label can be any element, so for example, the button
            -- can contain an image
            el
                [ centerX
                , centerY
                , width <| px 24
                , height <| px 24
                , paddingEach { top = 2, right = 0, bottom = 0, left = 2 }
                ]
            <|
                monoIconsSvg iconName 20 20 maybeStrcolor
        }


editableLabel : Model -> String -> Maybe msg -> Element msg
editableLabel model labelText maybeMsg =
    let
        colorScheme =
            getTheme model.theme
    in
    Input.button
        [ centerX
        , height <| px 30
        , Border.width 1
        , Border.rounded 5
        , Border.color colorScheme.mainBackground
        , mouseDown
            [ Border.color colorScheme.simpleButtonBorderFocused
            , Background.color colorScheme.simpleButtonBackgroundFocused
            ]
        , mouseOver
            [ Border.color colorScheme.simpleButtonBorderMouseOver
            , Background.color colorScheme.simpleButtonBackgroundMouseOver
            , Font.color <| rgb255 0xBE 0xCA 0xD6 -- TODO: Add into Dark theme
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
