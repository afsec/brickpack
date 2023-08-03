module Main exposing (main)

import Html exposing (..)
import Http
import Json.Decode as Decode
import Task exposing (Task)


type alias Model =
    { name : Maybe String
    , error : String
    }


type Msg
    = Name (Result String String)


eventsRequest : Http.Request (List String)
eventsRequest =
    Http.get "https://api.github.com/events"
        (Decode.at [ "actor", "login" ] Decode.string
            |> Decode.list
        )


nameRequest : String -> Http.Request String
nameRequest login =
    Http.get ("https://api.github.com/users/" ++ login)
        (Decode.at [ "name" ]
            (Decode.oneOf
                [ Decode.string
                , Decode.null "unspecified"
                ]
            )
        )


toHttpTask : Http.Request a -> Task String a
toHttpTask request =
    request
        |> Http.toTask
        |> Task.mapError toString


fetchEvents : Task String (List String)
fetchEvents =
    toHttpTask eventsRequest


fetchName : String -> Task String String
fetchName login =
    toHttpTask (nameRequest login)


pickFirst : List String -> Task String String
pickFirst logins =
    case List.head logins of
        Just login ->
            Task.succeed login

        Nothing ->
            Task.fail "No events."


init : ( Model, Cmd Msg )
init =
    { name = Nothing, error = "" }
        ! [ fetchEvents
                |> Task.andThen pickFirst
                |> Task.andThen fetchName
                |> Task.attempt Name
          ]


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Name (Ok name) ->
            { model | name = Just name } ! []

        Name (Err error) ->
            { model | error = error } ! []


view : Model -> Html Msg
view model =
    div []
        [ if model.error /= "" then
            div []
                [ h4 [] [ text "Error encountered" ]
                , pre [] [ text model.error ]
                ]
          else
            text ""
        , p [] [ text <| Maybe.withDefault "Fetching..." model.name ]
        ]


main =
    Html.program
        { init = init
        , update = update
        , subscriptions = always Sub.none
        , view = view
        }
