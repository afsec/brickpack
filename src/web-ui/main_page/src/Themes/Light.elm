module Themes.Light exposing (colorScheme)

import Element exposing (rgb255)
import Model exposing (Theme(..))


colorScheme : Theme -> { objType : Theme, mainBackground : Element.Color, mainFontColor : Element.Color, simpleButtonBorder : Element.Color, simpleButtonBorderMouseOver : Element.Color, simpleButtonBorderFocused : Element.Color, simpleButtonBackground : Element.Color, simpleButtonBackgroundMouseOver : Element.Color, simpleButtonBackgroundFocused : Element.Color, simpleButtonFontColor : Element.Color, simpleButtonFontColorMouseOver : Element.Color, basicPanelHeaderBackground : Element.Color, basicPanelContentBackground : Element.Color }
colorScheme theme =
    let
        _ =
            theme
    in
    { objType = ThemeLight
    , mainBackground = rgb255 0xFF 0xFF 0xFF
    , mainFontColor = rgb255 0x26 0x2B 0x31
    , simpleButtonBorder = rgb255 0xD5 0xD8 0xDA
    , simpleButtonBorderMouseOver = rgb255 0xD3 0xD5 0xD6
    , simpleButtonBorderFocused = rgb255 0xD3 0xD5 0xD6 -- TODO
    , simpleButtonBackground = rgb255 0xF6 0xF8 0xFA
    , simpleButtonBackgroundMouseOver = rgb255 0xF3 0xF4 0xF6

    -- , simpleButtonBackgroundFocused = rgb255 0x3D 0x44 0x4D -- TODO
    , simpleButtonBackgroundFocused = rgb255 0xFF 0x44 0x4D -- TODO
    , simpleButtonFontColor = rgb255 0x24 0x29 0x2F
    , simpleButtonFontColorMouseOver = rgb255 0x24 0x29 0x2F
    , basicPanelHeaderBackground = rgb255 0xFF 0x44 0x4D -- TODO
    , basicPanelContentBackground = rgb255 0xF6 0xF8 0xFA -- TODO
    }



-- colors =
--     { red = rgb255 0xFF 0x00 0x00
--     }
