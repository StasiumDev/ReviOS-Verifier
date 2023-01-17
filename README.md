[![Releases]](https://github.com/StasiumDev/ReviOS-Verifier/releases)

<a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-nc-nd/4.0/88x31.png" /></a><br />This work is licensed under a <a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/">Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International License</a>.

# A simple CLI Tool to Verify ReviOS ISO's
**How to Use:**

Drag and drop an ISO on top of the executable and let it run.

Once the process is done, you will be greeted with either message:
 
Successful verification will have green text saying:
>**Your SHA256 / MD5 hash matches with the official ReviOS ISO**

Unsuccessful verification will have red text saying:

>**Unable to find a matching SHA256 / MD5 hash!**

If your verification is unsuccessful, the tool will provide you with information on how to resolve this issue.

**Backstory why this Tool was made:**
>Due to some users having issues to use the built-in command, I came up with this Tool, not only is it much easier for the user to use, it also gets rid of the need to manually compare the hashes on the website.
