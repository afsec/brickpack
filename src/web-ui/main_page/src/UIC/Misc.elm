module UIC.Misc exposing (horizontalSpacer, verticalSeparator, verticalSpacer)

import Element
    exposing
        ( Element
        , el
        , fill
        , height
        , paddingXY
        , width
        )
import Element.Border as Border
import Model exposing (Model)
import Themes.Base exposing (getTheme)


horizontalSpacer : Element msg
horizontalSpacer =
    el [ width fill, paddingXY 10 1 ] Element.none


verticalSpacer : Element msg
verticalSpacer =
    el [ height fill, paddingXY 1 10 ] Element.none


verticalSeparator : Model -> Element msg
verticalSeparator model =
    let
        colorScheme =
            getTheme model.theme
    in
    el
        [ Border.widthEach { bottom = 0, left = 1, right = 0, top = 0 }
        , Border.color colorScheme.simpleButtonBorder
        , paddingXY 10 10
        ]
    <|
        Element.none
