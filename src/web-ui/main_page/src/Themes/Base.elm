module Themes.Base exposing (getTheme)

import Element exposing (Color)
import Model exposing (Theme(..))
import Themes.Dark as Dark
import Themes.Light as Light


getTheme : String -> { objType : Theme, mainBackground : Color, mainFontColor : Color, simpleButtonBorder : Color, simpleButtonBorderMouseOver : Color, simpleButtonBorderFocused : Color, simpleButtonBackground : Color, simpleButtonBackgroundMouseOver : Color, simpleButtonBackgroundFocused : Color, simpleButtonFontColor : Color, simpleButtonFontColorMouseOver : Color, basicPanelHeaderBackground : Color, basicPanelContentBackground : Color }
getTheme themeName =
    -- * `themeName` should be a string to support JSON external themes in the future
    case themeName of
        "dark" ->
            Dark.colorScheme ThemeDark

        "light" ->
            Light.colorScheme ThemeLight

        _ ->
            Light.colorScheme ThemeLight
