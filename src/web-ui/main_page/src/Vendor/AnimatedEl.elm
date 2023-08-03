module Vendor.AnimatedEl exposing (mySquare)

import Element exposing (..)
import Element.Background as Background
import Simple.Animation as Animation exposing (Animation)
import Simple.Animation.Animated as Animated
import Simple.Animation.Property as P


mySquare : Element msg
mySquare =
    animatedEl fade
        [ width (px 30)
        , height (px 30)
        , Background.color (rgb 1 0 0)
        ]
        none


fade : Animation
fade =
    Animation.fromTo
        { duration = 2000
        , options = []
        }
        [ P.opacity 1 ]
        [ P.opacity 0 ]


animatedEl : Animation -> List (Attribute msg) -> Element msg -> Element msg
animatedEl =
    animatedUi Element.el


animatedUi : (List (Attribute msg) -> children -> Element msg) -> Animation -> List (Attribute msg) -> children -> Element msg
animatedUi =
    Animated.ui
        { behindContent = Element.behindContent
        , htmlAttribute = Element.htmlAttribute
        , html = Element.html
        }



-- Reference: https://package.elm-lang.org/packages/andrewMacmurray/elm-simple-animation/2.3.0/
