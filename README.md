<a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-nc-nd/4.0/80x15.png"/></a>
This work is licensed under a <a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/">CC BY-NC-ND 4.0 International License</a>.


# A simple CLI Tool to Verify ReviOS ISO's
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/StasiumDev/ReviOS-Verifier/main.yml?logo=github&style=for-the-badge)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/StasiumDev/ReviOS-Verifier?style=for-the-badge&logo=rust&label=Version)
![GitHub top language](https://img.shields.io/github/languages/top/StasiumDev/ReviOS-Verifier?style=for-the-badge&logo=rust)
![GitHub](https://img.shields.io/github/license/StasiumDev/ReviOS-Verifier?style=for-the-badge&logo=gnu)
![GitHub All Releases](https://img.shields.io/github/downloads/StasiumDev/ReviOS-Verifier/total?style=for-the-badge&logo=gnu)
    

### How to Use:

Drag and drop an ISO on top of the executable and let it run.

<img src="https://cdn.discordapp.com/attachments/1064525050465763378/1065352767805329489/revi_verifier_demo.gif">

Once the process is done, you will be greeted with either message:
 
Successful verification will have green text saying:
>**Your SHA256 / MD5 hash matches with the official ReviOS ISO**

Unsuccessful verification will have red text saying:

>**Unable to find a matching SHA256 / MD5 hash!**

If your verification is unsuccessful, the tool will provide you with information on how to resolve this issue.


**Backstory why this Tool was made:**
>Due to some users having issues to use the built-in command, I came up with this Tool, not only is it much easier for the user to use, it also gets rid of the need to manually compare the hashes on the website.