module Themes.Dark exposing (colorScheme)

import Element exposing (rgb255)
import Model exposing (Theme(..))


colorScheme : Theme -> { objType : Theme, mainBackground : Element.Color, mainFontColor : Element.Color, simpleButtonBorder : Element.Color, simpleButtonBorderMouseOver : Element.Color, simpleButtonBorderFocused : Element.Color, simpleButtonBackground : Element.Color, simpleButtonBackgroundMouseOver : Element.Color, simpleButtonBackgroundFocused : Element.Color, simpleButtonFontColor : Element.Color, simpleButtonFontColorMouseOver : Element.Color, basicPanelHeaderBackground : Element.Color, basicPanelContentBackground : Element.Color }
colorScheme theme =
    let
        _ =
            theme
    in
    { objType = ThemeDark
    , mainBackground = rgb255 0x22 0x27 0x2E
    , mainFontColor = rgb255 0xCD 0xD9 0xE5
    , simpleButtonBorder = rgb255 0x46 0x4E 0x57
    , simpleButtonBorderMouseOver = rgb255 0x76 0x83 0x90
    , simpleButtonBorderFocused = rgb255 0x76 0x83 0x90
    , simpleButtonBackground = rgb255 0x37 0x3E 0x47
    , simpleButtonBackgroundMouseOver = rgb255 0x44 0x4C 0x56
    , simpleButtonBackgroundFocused = rgb255 0x3D 0x44 0x4D
    , simpleButtonFontColor = rgb255 0xAD 0xBA 0xC7
    , simpleButtonFontColorMouseOver = rgb255 0xAD 0xBA 0xC7
    , basicPanelHeaderBackground = rgb255 0x2D 0x33 0x3B
    , basicPanelContentBackground = rgb255 0x22 0x27 0x2E
    }



-- colors =
--     { red = rgb255 0xFF 0x00 0x00
--     }
