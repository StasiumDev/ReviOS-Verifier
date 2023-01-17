Copyright 2019-2023 Ricardo "Stasium" R.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at
    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

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
