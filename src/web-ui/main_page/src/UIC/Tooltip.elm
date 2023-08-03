module UIC.Tooltip exposing (myTooltip, tooltip)

import Element
    exposing
        ( Attribute
        , Element
        , el
        , fill
        , height
        , htmlAttribute
        , inFront
        , mouseOver
        , padding
        , rgb255
        , rgba
        , text
        , transparent
        , width
        )
import Element.Background as Background
import Element.Border as Border
import Element.Font as Font
import Html.Attributes
import Model exposing (Model)
import Themes.Base exposing (getTheme)



-- Reference: <https://gist.github.com/rlefevre/67a709b8d3cd80061b4b9beff6e80a0c>


myTooltip : Model -> String -> Element msg
myTooltip model tooltipText =
    let
        colorScheme =
            getTheme model.theme
    in
    el
        [ Background.color colorScheme.simpleButtonBackgroundMouseOver
        , Font.color <| rgb255 0xBE 0xCA 0xD6
        , padding 4
        , Border.rounded 5
        , Font.size 14
        , Border.shadow
            { offset = ( 0, 3 ), blur = 6, size = 0, color = rgba 0 0 0 0.32 }
        ]
        (text tooltipText)


tooltip : (Element msg -> Attribute msg) -> Element Never -> Attribute msg
tooltip usher tooltip_ =
    inFront <|
        el
            [ width fill
            , height fill
            , transparent True
            , mouseOver [ transparent False ]
            , (usher << Element.map never) <|
                el [ htmlAttribute (Html.Attributes.style "pointerEvents" "none") ]
                    tooltip_
            ]
            Element.none
