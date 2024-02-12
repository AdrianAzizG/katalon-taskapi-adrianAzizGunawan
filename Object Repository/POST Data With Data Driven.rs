<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Data With Data Driven</name>
   <tag></tag>
   <elementGuidId>f5d935fb-7aff-4a6b-9ce2-af72864594d7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;title\&quot;: \&quot;${title}\&quot;,\n    \&quot;body\&quot;: \&quot;${body}\&quot;,\n  \t\&quot;userId\&quot;: ${userId}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>44e63a11-dd4f-4066-ad9f-ce1147de1ed5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.url}/posts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'95'</defaultValue>
      <description></description>
      <id>3916912d-8159-4b7d-b12e-5783c489b223</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>'dune'</defaultValue>
      <description></description>
      <id>d29a9769-eb39-4585-a018-193bccc32e34</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <variables>
      <defaultValue>'big'</defaultValue>
      <description></description>
      <id>f5f608bb-e7f4-445a-8a53-5b571a191e3c</id>
      <masked>false</masked>
      <name>body</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
