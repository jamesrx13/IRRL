import os
import subprocess

from selenium import webdriver
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.chrome.service import Service
from webdriver_manager.chrome import ChromeDriverManager
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.common.by import By

import IRRL

#::: SETTINGS:::
MAIN_APP_URL = "https://procesojudicial.ramajudicial.gov.co/Justicia21/Administracion/Ciudadanos/frmConsulta.aspx"
PROFILE_PATH = os.path.abspath("chrome_profile") # If the reCaptchaV3 is blocking, try to use a other profile (Only need to change the folder name and delete the old one)
RUST_TOOL_PATH = "YOUR_RUST_TOOL_PATH_HERE" # Example: "C:/Users/username/Documents/IRRL.exe"

# :: DATA USING IN THIS EXAMPLE ::
DOCUMENT = "123456789"

# |-----------------------------------------
# | RPA EXAMPLE CODE USING IRRL + SELENIUM |           
# |-----------------------------------------

#::: CHROME OPTIONS:::
options = Options()

# Persisten profile
options.add_argument(f"--user-data-dir={PROFILE_PATH}")

# Stealth flags
options.add_argument("--disable-blink-features=AutomationControlled")
options.add_argument("--disable-infobars")
options.add_argument("--disable-notifications")
options.add_argument("--disable-extensions")
options.add_argument("--start-maximized")

# Define language
options.add_argument("--lang=es-CO")

# User-Agent
options.add_argument(
    "--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
    "AppleWebKit/537.36 (KHTML, like Gecko) "
    "Chrome/143.0.0.0 Safari/537.36"
)

# Disable bot detection
options.add_experimental_option("excludeSwitches", ["enable-automation"])
options.add_experimental_option('useAutomationExtension', False)

#::: DRIVER:::
driver = webdriver.Chrome(
    service=Service(ChromeDriverManager().install()),
    options=options
)

#::: STEALTH JAVASCRIPT:::
driver.execute_cdp_cmd(
    "Page.addScriptToEvaluateOnNewDocument",
    {
        "source": """
        Object.defineProperty(navigator, 'webdriver', {
            get: () => undefined
        });

        Object.defineProperty(navigator, 'languages', {
            get: () => ['es-CO', 'es']
        });

        Object.defineProperty(navigator, 'plugins', {
            get: () => [1, 2, 3, 4, 5]
        });

        const originalQuery = window.navigator.permissions.query;
        window.navigator.permissions.query = (parameters) =>
            parameters.name === 'notifications'
                ? Promise.resolve({ state: Notification.permission })
                : originalQuery(parameters);
        """
    }
)

#::: OPEN PAGE:::
driver.get(MAIN_APP_URL)

wait = WebDriverWait(driver, 20)

#::: --------------- DEF MAKE ACTIONS SEQUENCE --------------- :::
def fillDocument():
    input_document = wait.until(
        EC.element_to_be_clickable((By.ID, "MainContent_txtNumeroIdentificacion"))
    )
    input_document.send_keys(DOCUMENT)  

functionArray = [
    fillDocument,
]

# ::: CALL THE RUST TOOL :::
process = subprocess.Popen(
    [RUST_TOOL_PATH, "replay", "tyba"] # This will replay the events recorded in the "tyba" folder (Use the other name if you used a different one)
)

#::: IRRL ACTIONS RPA:::
irrl = IRRL.IRRL()
irrl.actionsRpa(functionArray) # This method will block until all actions are done

#::: DISCONNECT IRRL AND DRIVER :::
irrl.irrlDisconnect()
driver.quit()
