module Vendor.AnimatedCustom exposing (mySquare)

import Element exposing (..)
import Element.Background as Background
import Element.Border as Border
import Simple.Animation as Animation exposing (Animation)
import Simple.Animation.Animated as Animated
import Simple.Animation.Property as AnimationProperty


mySquare : Element msg
mySquare =
    Animated.ui
        { behindContent = Element.behindContent
        , htmlAttribute = Element.htmlAttribute
        , html = Element.html
        }
        Element.el
        okBorder
        [ width <| px 30
        , height <| px 30
        , Background.color <| rgb255 0xA1 0xA1 0xA1
        , Border.width 2

        -- , Border.color <| rgb255 0xFF 0x00 0x00
        ]
        none


okBorder : Animation
okBorder =
    Animation.fromTo
        { duration = 2000
        , options = []
        }
        [ AnimationProperty.borderColor "#00FF00" ]
        [ AnimationProperty.borderColor "#000000" ]


errorBorder : Animation
errorBorder =
    Animation.fromTo
        { duration = 2000
        , options = []
        }
        [ AnimationProperty.borderColor "#FF0000" ]
        [ AnimationProperty.borderColor "#000000" ]
