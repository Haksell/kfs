import pyperclip

art = """\
      :::    ::: :::::::::: ::::::::            :::     ::::::::
     :+:   :+:  :+:       :+:    :+:          :+:     :+:    :+:
    +:+  +:+   +:+       +:+                +:+ +:+        +:+  
   +#++:++    :#::+::#  +#++:++#++        +#+  +:+      +#+     
  +#+  +#+   +#+              +#+       +#+#+#+#+#+  +#+        
 #+#   #+#  #+#       #+#    #+#             #+#   #+#          
###    ### ###        ########              ###  ##########     
"""

banner = "".join(
    '        Self::print_welcome_title(b"'
    + "".join(f"\{hex(c)[1:]}" for c in line.encode("cp437"))
    + '");\n'
    for line in art.strip("\n").split("\n")
)

print(banner)
pyperclip.copy(banner)
