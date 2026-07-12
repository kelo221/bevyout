[bevy](../../index.html)::[post\_process](../index.html)

# Module auto\_exposure 

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/lib.rs.html#9)

## Structs

[AutoExposure](struct.AutoExposure.html "struct bevy::post_process::auto_exposure::AutoExposure")

Component that enables auto exposure for an HDR-enabled 2d or 3d camera.

[AutoExposureCompensationCurve](struct.AutoExposureCompensationCurve.html "struct bevy::post_process::auto_exposure::AutoExposureCompensationCurve")

An auto exposure compensation curve. This curve is used to map the average log luminance of a scene to an exposure compensation value, to allow for fine control over the final exposure.

[AutoExposurePlugin](struct.AutoExposurePlugin.html "struct bevy::post_process::auto_exposure::AutoExposurePlugin")

Plugin for the auto exposure feature.

## Enums

[AutoExposureCompensationCurveError](enum.AutoExposureCompensationCurveError.html "enum bevy::post_process::auto_exposure::AutoExposureCompensationCurveError")

Various errors that can occur when constructing an [`AutoExposureCompensationCurve`](struct.AutoExposureCompensationCurve.html "struct bevy::post_process::auto_exposure::AutoExposureCompensationCurve").

## Functions

[init\_auto\_exposure\_resources](fn.init_auto_exposure_resources.html "fn bevy::post_process::auto_exposure::init_auto_exposure_resources")