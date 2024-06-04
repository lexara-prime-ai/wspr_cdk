import constants
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
        print("\n[ERROR] -> Failed to [Authenticate] <wspr_cdk> service user: \n", e)


def upload_to_drive(file_path):
    try:
        credentials = authenticate()
        service = build("drive", "v3", credentials=credentials)

        file_metadata = {
            "name": "wspr_spot_data.csv",
            "parents": [constants.PARENT_FOLDER_ID],
        }

        print("[Uploading] file to Google Drive...\n")

        service.files().create(body=file_metadata, media_body=file_path).execute()
    except Exception as e:
        print("\n[ERROR] -> Failed to upload to Google Drive: \n", e)
