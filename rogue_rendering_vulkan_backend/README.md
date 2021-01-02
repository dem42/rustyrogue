# VulkanSDK

note that some layers that we use here come from the vulkanSDK and not the vulkan runtime

for example we make use of the validation layer VK_LAYER_KHRONOS_validation
for this we need the vulkanSDK which has this layer and for this we should have the 
VK_LAYER_PATH set in environment variables

you can use 
```
> vulkaninfo 
```
to see your layers