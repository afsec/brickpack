port module Main exposing (main)

import Base64
import Browser
import Browser.Events
import Data.HttpDataAppletArtifact exposing (jsonDecodeAppletArtifact)
import Data.HttpDataApplets exposing (jsonDecodeAppletEntryList)
import Dict
import Html exposing (Html)
import Http
import Json.Decode exposing (errorToString)
import Layout.Base exposing (renderLayout)
import Model
    exposing
        ( AppletArtifactStatus(..)
        , AppletEntryListStatus(..)
        , Flags
        , Model
        , Msg(..)
        , Page(..)
        , RepoFromBackend(..)
        , ResultButtonState(..)
        , SessionState(..)
        , repoFromBackendToUrlString
        , serverUrl
        )
import Task
import Time
import Vendor.Helpers
    exposing
        ( expectStringDetailed
        , httpErrorToString
        , sortAppletEntryListByfilename
        )



---- MAIN ----


main : Program Flags Model Msg
main =
    Browser.element
        { view = view
        , init = init
        , update = update
        , subscriptions = subscriptions
        }



---- SUBSCRIPTIONS ----


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.batch
        [ -- receiveCodeFromMonacoEditor CodeReceivedFromEditor
          Browser.Events.onResize OnResize
        ]



---- PORTS ----


port consoleLog : String -> Cmd msg


port consoleError : String -> Cmd msg


port newBrowserTab : String -> Cmd msg



---- MODEL ----


init : Flags -> ( Model, Cmd Msg )
init flags =
    update MainPage
        { error = Nothing
        , header_panel =
            { refresh_btn = ResultButtonNoOp
            , applets = Nothing
            , users = Nothing
            , departments = Nothing
            , permissions = Nothing
            , statuses = Nothing
            }
        , logon_buffer =
            -- { username = Nothing
            -- , password = Nothing
            -- }
            -- TODO: Mocking up
            { username = Just "user8732"
            , password = Just "pass9164"
            }
        , pages =
            { current = NoPage
            , applets =
                { artifact = DataAppletArtifactNoRequest
                , code_buffer = Nothing
                , filename_buffer = Nothing
                , selected = Nothing
                , list = DataAppletEntryListNoRequest
                }
            , users = Nothing
            , departments = Nothing
            , permissions = Nothing
            , statuses = Nothing
            }
        , session = NoSession
        , theme = "dark"
        , timezone = Nothing
        , ua = flags.ua
        , window_size_x = flags.x
        , window_size_y = flags.y
        }



---- UPDATE ----


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    let
        header_panel =
            model.header_panel

        -- Pages BEGIN
        pages =
            model.pages

        applets =
            model.pages.applets

        -- Pages END
    in
    case msg of
        NoOp ->
            ( model, Cmd.none )

        MainPage ->
            ( { model | pages = { pages | current = NoPage } }
            , Cmd.batch
                [ Task.perform GetHeaderData (Task.succeed ())
                , Task.perform AdjustTimeZoneGet (Task.succeed ())
                ]
            )

        AdjustTimeZoneGet _ ->
            ( model
            , Task.perform AdjustTimeZone Time.here
            )

        AdjustTimeZone timeZone ->
            ( { model | timezone = Just timeZone }
            , Cmd.none
            )

        -- Logon BEGIN
        UserTypedUsername typedUsername ->
            let
                logon_buffer =
                    model.logon_buffer
            in
            ( { model | logon_buffer = { logon_buffer | username = Just typedUsername } }
            , Cmd.none
            )

        UserTypedPassword typedPassword ->
            let
                logon_buffer =
                    model.logon_buffer
            in
            ( { model | logon_buffer = { logon_buffer | username = Just typedPassword } }
            , Cmd.none
            )

        StartSession ->
            --TODO: Currently if just a mockup
            let
                logon_buffer =
                    model.logon_buffer

                maybeUsername =
                    logon_buffer.username

                maybePassword =
                    logon_buffer.password

                checkCredentials username password =
                    if username == "user8732" && password == "pass9164" then
                        Just "someCraftedToken"

                    else
                        Nothing

                doLogon =
                    Maybe.map2 checkCredentials maybeUsername maybePassword

                tryLogon =
                    case doLogon of
                        Just maybeToken ->
                            case maybeToken of
                                Just token ->
                                    Ok token

                                Nothing ->
                                    Err "Unable to get token"

                        Nothing ->
                            Err "Unable to log in"
            in
            case tryLogon of
                Ok token ->
                    ( { model
                        | session = SessionSuccess token
                        , logon_buffer =
                            { logon_buffer | username = Nothing, password = Nothing }
                      }
                    , Cmd.none
                    )

                Err error ->
                    ( { model | session = SessionFailure error }, Cmd.none )

        EndSession ->
            -- TODO
            ( { model | session = SessionLogout }
            , Cmd.none
            )

        -- Logon END
        ApplicationErrorShow ->
            case model.error of
                Just errorString ->
                    ( model, consoleError errorString )

                Nothing ->
                    ( model, consoleError "Error without string. That's wierd!" )

        OpenNewBrowserTab url ->
            ( model, newBrowserTab url )

        ShowPage page ->
            case page of
                PageApplets ->
                    update (GetAppletList ()) { model | pages = { pages | current = page } }

                -- TODO: Implement for other pages
                _ ->
                    ( { model | pages = { pages | current = page } }, Cmd.none )

        OnResize windowSizeX windowSizeY ->
            ( { model | window_size_x = windowSizeX, window_size_y = windowSizeY }
            , consoleLog <|
                String.concat
                    [ "Window resize detected!"
                    , "[ "
                    , String.fromInt windowSizeX
                    , " x "
                    , String.fromInt windowSizeY
                    , " ]"
                    ]
            )

        GetHeaderData _ ->
            ( { model
                | header_panel =
                    { refresh_btn = ResultButtonNoOp
                    , applets = Nothing
                    , users = Nothing
                    , departments = Nothing
                    , permissions = Nothing
                    , statuses = Nothing
                    }
              }
            , Cmd.batch
                [ Task.perform GetRepoCount (Task.succeed RepoUsers)
                , Task.perform GetRepoCount (Task.succeed RepoDepartments)
                , Task.perform GetRepoCount (Task.succeed RepoPermissions)
                , Task.perform GetRepoCount (Task.succeed RepoStatuses)
                , Task.perform GetRepoCount (Task.succeed RepoApplets)
                ]
            )

        GetRepoCount repo ->
            let
                repoString =
                    repoFromBackendToUrlString repo
            in
            ( model
            , Http.request
                { body = Http.emptyBody
                , expect = expectStringDetailed (GotRepoCount repo)
                , headers = []
                , method = "HEAD"
                , timeout = Nothing
                , tracker = Nothing
                , url = serverUrl ++ "/" ++ repoString
                }
            )

        GotRepoCount repo httpResponse ->
            case httpResponse of
                -- Ok ( metadata, body ) ->
                Ok ( metadata, _ ) ->
                    let
                        headers =
                            metadata.headers

                        maybeRetrievedCount =
                            Dict.get "x-total-count" headers
                                |> Maybe.andThen String.toInt
                    in
                    case repo of
                        RepoApplets ->
                            case maybeRetrievedCount of
                                Just retrievedCount ->
                                    ( { model
                                        | header_panel =
                                            { header_panel
                                                | applets = Just retrievedCount
                                            }
                                      }
                                    , Cmd.none
                                    )

                                Nothing ->
                                    ( { model
                                        | header_panel =
                                            { header_panel
                                                | applets = Nothing
                                            }
                                      }
                                    , consoleError "Error on parsing Integer from x-total-count"
                                    )

                        RepoDepartments ->
                            case maybeRetrievedCount of
                                Just retrievedCount ->
                                    ( { model
                                        | header_panel =
                                            { header_panel
                                                | departments = Just retrievedCount
                                            }
                                      }
                                    , Cmd.none
                                    )

                                Nothing ->
                                    ( { model
                                        | header_panel =
                                            { header_panel
                                                | departments = Nothing
                                            }
                                      }
                                    , consoleError "Error on parsing Integer from x-total-count"
                                    )

                        RepoPermissions ->
                            case maybeRetrievedCount of
                                Just retrievedCount ->
                                    ( { model
                                        | header_panel =
                                            { header_panel
                                                | permissions = Just retrievedCount
                                            }
                                      }
                                    , Cmd.none
                                    )

                                Nothing ->
                                    ( { model
                                        | header_panel =
                                            { header_panel
                                                | permissions = Nothing
                                            }
                                      }
                                    , consoleError "Error on parsing Integer from x-total-count"
                                    )

                        RepoStatuses ->
                            case maybeRetrievedCount of
                                Just retrievedCount ->
                                    ( { model
                                        | header_panel =
                                            { header_panel
                                                | statuses = Just retrievedCount
                                            }
                                      }
                                    , Cmd.none
                                    )

                                Nothing ->
                                    ( { model
                                        | header_panel =
                                            { header_panel
                                                | statuses = Nothing
                                            }
                                      }
                                    , consoleError "Error on parsing Integer from x-total-count"
                                    )

                        RepoUsers ->
                            case maybeRetrievedCount of
                                Just retrievedCount ->
                                    ( { model
                                        | header_panel =
                                            { header_panel
                                                | users = Just retrievedCount
                                            }
                                      }
                                    , Cmd.none
                                    )

                                Nothing ->
                                    ( { model
                                        | header_panel =
                                            { header_panel
                                                | users = Nothing
                                            }
                                      }
                                    , consoleError "Error on parsing Integer from x-total-count"
                                    )

                Err httpError ->
                    ( { model
                        | header_panel =
                            { header_panel
                                | refresh_btn = ResultButtonClickedErr
                            }
                      }
                    , consoleError <| httpErrorToString httpError
                    )

        RefreshAppletList ->
            ( model
            , Cmd.batch
                [ Task.perform GetAppletList (Task.succeed ())
                , Task.perform GetRepoCount (Task.succeed RepoApplets)
                ]
            )

        GetAppletList _ ->
            ( { model
                | pages =
                    { pages
                        | applets = { applets | list = DataAppletEntryListRequestPending }
                    }
              }
            , Http.get
                { url = serverUrl ++ "/applets"
                , expect = Http.expectString GotAppletList -- * Go to next state: `GotAppletList` *
                }
            )

        GotAppletList result ->
            case result of
                Ok json ->
                    case jsonDecodeAppletEntryList json of
                        Ok data ->
                            ( { model
                                | pages =
                                    { pages
                                        | applets =
                                            { applets
                                                | list =
                                                    DataAppletEntryListRequestSuccess
                                                        (sortAppletEntryListByfilename data)
                                            }
                                    }
                              }
                            , Cmd.none
                            )

                        Err error ->
                            update ApplicationErrorShow
                                { model | error = Just (errorToString error) }

                Err _ ->
                    ( { model
                        | pages =
                            { pages
                                | applets =
                                    { applets
                                        | list =
                                            DataAppletEntryListRequestFailure
                                    }
                            }
                      }
                    , Cmd.none
                    )

        GetAppletArtifact appletArtifactId ->
            ( { model
                | pages =
                    { pages
                        | applets =
                            { applets
                                | code_buffer = Nothing
                                , filename_buffer = Nothing
                                , artifact = DataAppletArtifactRequestPending
                            }
                    }
              }
            , Http.get
                { url = serverUrl ++ "/applets/" ++ appletArtifactId
                , expect = Http.expectString GotAppletArtifact -- * Go to next state: `GotAppletArtifact` *
                }
            )

        GotAppletArtifact result ->
            case result of
                Ok json ->
                    case jsonDecodeAppletArtifact json of
                        Ok appletArtifact ->
                            case Base64.decode appletArtifact.code of
                                Ok code ->
                                    ( { model
                                        | pages =
                                            { pages
                                                | applets =
                                                    { applets
                                                        | code_buffer = Just code
                                                        , artifact = DataAppletArtifactRequestSuccess appletArtifact
                                                    }
                                            }
                                      }
                                    , Cmd.none
                                    )

                                Err error ->
                                    update ApplicationErrorShow { model | error = Just error }

                        Err error ->
                            update ApplicationErrorShow { model | error = Just (errorToString error) }

                Err error ->
                    update ApplicationErrorShow
                        { model
                            | pages =
                                { pages
                                    | applets =
                                        { applets
                                            | artifact = DataAppletArtifactRequestFailure
                                            , code_buffer = Nothing
                                        }
                                }
                            , error = Just (httpErrorToString error)
                        }

        EditorNewApplet ->
            -- TODO
            let
                maybeDefaultFilename =
                    Just "new_code.lua"

                maybeEncodedCode =
                    Just (Base64.encode " ")

                sendRequest defaultFilename defaultCodeInBase64 =
                    ( model
                    , Http.request
                        { body =
                            Http.stringBody "application/json" <|
                                String.concat
                                    [ "{"
                                    , "\"filename\": \"" ++ defaultFilename ++ "\","
                                    , "\"code\": \"" ++ defaultCodeInBase64 ++ "\""
                                    , "}"
                                    ]
                        , expect = Http.expectString EditorNewAppletSent
                        , headers = []
                        , method = "POST"
                        , timeout = Nothing
                        , tracker = Nothing
                        , url = serverUrl ++ "/applets"
                        }
                    )
            in
            case Maybe.map2 sendRequest maybeDefaultFilename maybeEncodedCode of
                Just outcome ->
                    outcome

                Nothing ->
                    -- TODO: Define a default behavior for IMPOSSIBLE_STATEs
                    ( model, Cmd.none )

        EditorNewAppletSent result ->
            -- TODO: Review needed
            ( { model
                | pages =
                    { pages
                        | applets =
                            { applets
                                | selected = Nothing
                                , artifact = DataAppletArtifactNoRequest
                                , code_buffer = Nothing
                            }
                    }
              }
            , Cmd.batch
                [ Task.perform GetAppletList (Task.succeed ())
                , Task.perform GetRepoCount (Task.succeed RepoApplets)
                ]
            )

        EditorSaveApplet ->
            -- TODO: Still needs to create a Request state controller
            let
                maybeAppletId =
                    applets.selected
                        |> Maybe.andThen (\applet -> Just applet.oid)

                maybeEncodedCode =
                    applets.code_buffer
                        |> Maybe.andThen (\code -> Just (Base64.encode code))

                sendRequest appletId codeInBase64 =
                    ( model
                    , Http.request
                        { body =
                            Http.stringBody "application/json" <|
                                String.concat
                                    [ "{"
                                    , "\"code\": \"" ++ codeInBase64 ++ "\""
                                    , "}"
                                    ]
                        , expect = Http.expectString EditorSavedApplet
                        , headers = []
                        , method = "PATCH"
                        , timeout = Nothing
                        , tracker = Nothing
                        , url = serverUrl ++ "/applets/" ++ appletId
                        }
                    )
            in
            case Maybe.map2 sendRequest maybeAppletId maybeEncodedCode of
                Just outcome ->
                    outcome

                Nothing ->
                    -- TODO: Define a default behavior for IMPOSSIBLE_STATEs
                    ( model, Cmd.none )

        EditorSavedApplet result ->
            -- TODO: Get `appletArtifactId` from `result`
            let
                maybeAppletArtifactId =
                    Maybe.andThen (\selected -> Just selected.oid) applets.selected
            in
            case maybeAppletArtifactId of
                Just appletArtifactId ->
                    ( { model
                        | pages =
                            { pages
                                | applets =
                                    { applets
                                        | artifact = DataAppletArtifactNoRequest
                                        , code_buffer = Nothing
                                        , filename_buffer = Nothing
                                    }
                            }
                      }
                    , Cmd.batch
                        [ Task.perform GetAppletArtifact (Task.succeed appletArtifactId)
                        , Task.perform GetAppletList (Task.succeed ())
                        ]
                    )

                Nothing ->
                    -- TODO: Define a default behavior for IMPOSSIBLE_STATEs
                    ( model, Cmd.none )

        ------------
        EditorRenameApplet newFilename ->
            ( { model
                | pages =
                    { pages
                        | applets =
                            { applets
                                | filename_buffer = Just newFilename
                            }
                    }
              }
            , Cmd.none
            )

        EditorRenameAppletCancel ->
            ( { model
                | pages =
                    { pages
                        | applets =
                            { applets
                                | filename_buffer = Nothing
                            }
                    }
              }
            , Cmd.none
            )

        EditorRenameAppletSend ->
            -- TODO
            let
                maybeAppletId =
                    applets.selected
                        |> Maybe.andThen (\applet -> Just applet.oid)

                maybeFilenameBuffer =
                    applets.filename_buffer

                sendRequest appletId filenameBuffer =
                    ( model
                    , Http.request
                        { body =
                            Http.stringBody "application/json" <|
                                String.concat
                                    [ "{"
                                    , "\"filename\": \"" ++ filenameBuffer ++ "\""
                                    , "}"
                                    ]
                        , expect = Http.expectString EditorRenamedApplet
                        , headers = []
                        , method = "PATCH"
                        , timeout = Nothing
                        , tracker = Nothing
                        , url = serverUrl ++ "/applets/" ++ appletId
                        }
                    )
            in
            case Maybe.map2 sendRequest maybeAppletId maybeFilenameBuffer of
                Just outcome ->
                    outcome

                Nothing ->
                    -- TODO: Define a default behavior for IMPOSSIBLE_STATEs
                    ( model, Cmd.none )

        EditorRenamedApplet result ->
            -- TODO
            let
                maybeAppletArtifactId =
                    Maybe.andThen (\selected -> Just selected.oid) applets.selected
            in
            case maybeAppletArtifactId of
                Just appletArtifactId ->
                    ( { model
                        | pages =
                            { pages
                                | applets =
                                    { applets
                                        | artifact = DataAppletArtifactNoRequest
                                        , code_buffer = Nothing
                                        , filename_buffer = Nothing
                                    }
                            }
                      }
                    , Cmd.batch
                        [ Task.perform GetAppletArtifact (Task.succeed appletArtifactId)
                        , Task.perform GetAppletList (Task.succeed ())
                        ]
                    )

                Nothing ->
                    -- TODO: Define a default behavior for IMPOSSIBLE_STATEs
                    ( model, Cmd.none )

        ----------
        EditorDeleteApplet appletId ->
            -- TODO
            ( model
            , Http.request
                { body =
                    Http.emptyBody
                , expect = Http.expectString EditorNewAppletSent
                , headers = []
                , method = "DELETE"
                , timeout = Nothing
                , tracker = Nothing
                , url = serverUrl ++ "/applets/" ++ appletId
                }
            )

        EditorDeletedApplet result ->
            -- TODO: Review needed
            update (GetAppletList ())
                { model
                    | pages =
                        { pages
                            | applets =
                                { applets
                                    | selected = Nothing
                                    , artifact = DataAppletArtifactNoRequest
                                    , code_buffer = Nothing
                                }
                        }
                }

        EditorOpenApplet appletEntry ->
            update (GetAppletArtifact appletEntry.oid)
                { model
                    | pages =
                        { pages
                            | applets =
                                { applets
                                    | selected = Just appletEntry
                                }
                        }
                }

        EditorCloseApplet ->
            ( { model
                | pages =
                    { pages
                        | applets =
                            { applets
                                | selected = Nothing
                                , artifact = DataAppletArtifactNoRequest
                                , code_buffer = Nothing
                                , filename_buffer = Nothing
                            }
                    }
              }
            , Cmd.none
            )

        UpdateCodeBuffer code ->
            ( { model
                | pages =
                    { pages
                        | applets =
                            { applets
                                | code_buffer = Just code
                            }
                    }
              }
            , Cmd.none
            )

        ToggleTheme ->
            if model.theme == "dark" then
                ( { model | theme = "light" }, Cmd.none )

            else
                ( { model | theme = "dark" }, Cmd.none )



---- VIEW ----


view : Model -> Html Msg
view model =
    renderLayout model
