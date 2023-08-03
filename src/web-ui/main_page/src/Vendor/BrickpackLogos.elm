module Vendor.BrickpackLogos exposing (brickpackLogo)

import Element exposing (Element)
import Svg exposing (svg)
import Svg.Attributes as SvgAttribute
import VirtualDom


brickpackLogo : Int -> Int -> Maybe String -> Element msg
brickpackLogo width height maybeFill =
    -- TODO: Check on Element.image to get some insights about style normalization-like code.
    -- TODO: ?Is there some Baseline os Guidelines for custom/new Elm-ui elements?
    let
        colorStr =
            case maybeFill of
                Just rgbStr ->
                    rgbStr

                Nothing ->
                    "#0D0D0D"
    in
    Element.html <|
        svg
            [ SvgAttribute.width <| String.fromInt width
            , SvgAttribute.height <| String.fromInt height
            , SvgAttribute.viewBox "0 0 190 150"
            , SvgAttribute.fill "none"
            , VirtualDom.attribute "xmlns" "http://www.w3.org/2000/svg"
            ]
            [ Svg.rect
                [ SvgAttribute.x "10"
                , SvgAttribute.y "10"
                , SvgAttribute.rx "5"
                , SvgAttribute.ry "5"
                , SvgAttribute.width "170"
                , SvgAttribute.height "130"
                , SvgAttribute.stroke colorStr
                , SvgAttribute.fill "transparent"
                , SvgAttribute.strokeWidth "15"
                ]
                []
            , Svg.line
                [ SvgAttribute.x1 "10"
                , SvgAttribute.y1 "75"
                , SvgAttribute.x2 "180"
                , SvgAttribute.y2 "75"
                , SvgAttribute.stroke colorStr
                , SvgAttribute.strokeWidth "15"
                ]
                []
            , Svg.line
                [ SvgAttribute.x1 "10"
                , SvgAttribute.y1 "75"
                , SvgAttribute.x2 "180"
                , SvgAttribute.y2 "75"
                , SvgAttribute.stroke colorStr
                , SvgAttribute.strokeWidth "15"
                ]
                []
            , Svg.line
                [ SvgAttribute.x1 "70"
                , SvgAttribute.y1 "10"
                , SvgAttribute.x2 "70"
                , SvgAttribute.y2 "80"
                , SvgAttribute.stroke colorStr
                , SvgAttribute.strokeWidth "15"
                ]
                []
            , Svg.line
                [ SvgAttribute.x1 "120"
                , SvgAttribute.y1 "80"
                , SvgAttribute.x2 "120"
                , SvgAttribute.y2 "140"
                , SvgAttribute.stroke colorStr
                , SvgAttribute.strokeWidth "15"
                ]
                []
            ]



--   <line x1="120" y1="80" x2="120" y2="140" stroke="white" stroke-width="15" />
