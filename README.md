
# A simple CLI Tool to Verify ReviOS ISO's
[![GitHub License](https://img.shields.io/static/v1?label=LICENSE&message=CC%20BY-NC-ND&color=188af5&style=for-the-badge&logo=creativecommons&logoColor=fff)](http://creativecommons.org/licenses/by-nc-nd/4.0/)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/StasiumDev/ReviOS-Verifier/main.yml?logo=github&style=for-the-badge)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/StasiumDev/ReviOS-Verifier?style=for-the-badge&logo=rust&label=Version&color=188af5)](https://github.com/StasiumDev/ReviOS-Verifier/releases)
![GitHub All Releases](https://img.shields.io/github/downloads/StasiumDev/ReviOS-Verifier/total?style=for-the-badge&logo=Google%20Chrome&color=188af5)
 
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