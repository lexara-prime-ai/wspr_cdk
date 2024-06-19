# Terminal colors.
CRED = "\033[91m"
CEND = "\033[0m"


 # -----------------------------------
"""Custom <error> handling."""

# -----------------------------------
class ErrorHandling:
    def propagate_error(self, process_name: str, message: str):
        print(
            f"""
        {CRED} An [ERROR] occured {CEND}          
        
        Info: {message}
        Process: {process_name}
     """
        )