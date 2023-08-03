module Model exposing
    ( AppletArtifact
    , AppletArtifactStatus(..)
    , AppletEntry
    , AppletEntryListStatus(..)
    , Flags
    , HeaderPanelModel
    , LogonForm
    , Model
    , Msg(..)
    , Page(..)
    , RepoFromBackend(..)
    , ResultButtonState(..)
    , SessionState(..)
    , Theme(..)
    , repoFromBackendToUrlString
    , serverUrl
    )

import Http
import Time



---- MSG ----


type Msg
    = NoOp
    | AdjustTimeZoneGet ()
    | AdjustTimeZone Time.Zone
    | MainPage
    | ToggleTheme
    | OnResize Int Int
    | UpdateCodeBuffer String
    | ApplicationErrorShow
      -- Logon
    | UserTypedUsername String
    | UserTypedPassword String
    | StartSession
    | EndSession
      -- Header panel
    | GetHeaderData () -- Batch [GetRepoCount *All Repos*]
    | GetRepoCount RepoFromBackend
    | GotRepoCount RepoFromBackend (Result Http.Error ( Http.Metadata, String ))
    | ShowPage Page
      -- Applets
    | RefreshAppletList -- Batch [GetRepoCount Applets, GetAppletList]
    | GetAppletList ()
    | GotAppletList (Result Http.Error String)
    | EditorNewApplet
    | EditorNewAppletSent (Result Http.Error String)
    | OpenNewBrowserTab String
    | EditorSaveApplet
    | EditorSavedApplet (Result Http.Error String)
    | EditorDeleteApplet String
    | EditorDeletedApplet (Result Http.Error String)
    | EditorRenameApplet String
    | EditorRenameAppletCancel
    | EditorRenameAppletSend
    | EditorRenamedApplet (Result Http.Error String)
    | EditorOpenApplet AppletEntry
    | EditorCloseApplet
    | GetAppletArtifact String
    | GotAppletArtifact (Result Http.Error String)



---- MODEL ----


type alias Model =
    { error : Maybe String
    , header_panel : HeaderPanelModel
    , logon_buffer : LogonForm
    , pages : PagesModel
    , session : SessionState
    , theme : String
    , timezone : Maybe Time.Zone
    , ua : String
    , window_size_x : Int
    , window_size_y : Int
    }


type alias HeaderPanelModel =
    { refresh_btn : ResultButtonState
    , applets : Maybe Int
    , users : Maybe Int
    , departments : Maybe Int
    , permissions : Maybe Int
    , statuses : Maybe Int
    }


type alias PagesModel =
    { current : Page
    , applets : PageAppletsData
    , users : Maybe ()
    , departments : Maybe ()
    , permissions : Maybe ()
    , statuses : Maybe ()
    }


type Page
    = NoPage
    | PageApplets
    | PageDepartments
    | PagePermissions
    | PageStatuses
    | PageUsers


type alias PageAppletsData =
    { artifact : AppletArtifactStatus
    , code_buffer : Maybe String
    , filename_buffer : Maybe String
    , selected : Maybe AppletEntry
    , list : AppletEntryListStatus
    }


type alias LogonForm =
    { username : Maybe String
    , password : Maybe String
    }



----------------
---- CONSTANTS ----


serverUrl : String
serverUrl =
    "/api"



---- FLAGS ----


type alias Flags =
    { x : Int
    , y : Int
    , ua : String
    }



---- GLOBAL VARIABLES ----


type Theme
    = ThemeDark
    | ThemeLight



--------------------------------


type RepoFromBackend
    = RepoApplets
    | RepoDepartments
    | RepoPermissions
    | RepoStatuses
    | RepoUsers


repoFromBackendToUrlString : RepoFromBackend -> String
repoFromBackendToUrlString repoFromBackend =
    case repoFromBackend of
        RepoApplets ->
            "applets"

        RepoDepartments ->
            "departments"

        RepoPermissions ->
            "permissions"

        RepoStatuses ->
            "statuses"

        RepoUsers ->
            "users"



-----------------------------------------


type alias AppletEntry =
    { oid : String
    , filename : String
    , size : Int
    , created_at : Int
    , updated_at : Int
    }


type alias AppletArtifact =
    { oid : String
    , filename : String
    , code : String
    , size : Int
    , created_at : Int
    , updated_at : Int
    }


type AppletEntryListStatus
    = DataAppletEntryListNoRequest
    | DataAppletEntryListRequestPending
    | DataAppletEntryListRequestFailure
    | DataAppletEntryListRequestSuccess (List AppletEntry)


type AppletArtifactStatus
    = DataAppletArtifactNoRequest
    | DataAppletArtifactRequestPending
    | DataAppletArtifactRequestFailure
    | DataAppletArtifactRequestSuccess AppletArtifact


type ResultButtonState
    = ResultButtonNoOp
    | ResultButtonClickedOk
    | ResultButtonClickedErr


type SessionState
    = NoSession
    | SessionProcessing
    | SessionFailure String
    | SessionSuccess String
    | SessionLogout
