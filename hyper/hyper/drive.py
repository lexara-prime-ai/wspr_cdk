# Utility modules.
import constants
import error_handling

# Google deps.
from google.oauth2 import service_account
from googleapiclient.discovery import build


def authenticate():
    try:
        print("\n[Authenticating] <wspr_cdk> service user...")

        credentials = service_account.Credentials.from_service_account_file(
            constants.SERVICE_ACCOUNT_FILE, scopes=constants.SCOPES
        )

        return credentials
    except Exception as e:
        error_handling.ErrorHandling.propagate_error(
            f"{authenticate.__name__}",
            f"\n[ERROR] -> Failed to [Authenticate] <wspr_cdk> service user: {e}\n"
        )


def upload_to_drive(file_path):
    try:
        credentials = authenticate()
        service = build("drive", "v3", credentials=credentials)

        file_metadata = {
            "name": constants.FILE_NAME.strip(),
            "parents": [constants.PARENT_FOLDER_ID],
        }

        print("[Uploading] file to Google Drive...\n")
        print("\n[Waiting]\nPress CTRL + C to exit...\n")

        service.files().create(body=file_metadata, media_body=file_path).execute()
    except Exception as e:
        error_handling.ErrorHandling.propagate_error(
            f"{upload_to_drive.__name__}",
            f"\n[ERROR] -> Failed to upload to Google Drive: {e}\n"
        )
