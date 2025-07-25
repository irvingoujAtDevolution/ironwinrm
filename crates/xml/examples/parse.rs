const SOAP: &str = r#"
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope"
            xmlns:wsa="http://schemas.xmlsoap.org/ws/2004/08/addressing"
            xmlns:wsman="http://schemas.dmtf.org/wbem/wsman/1/wsman.xsd">
  <s:Header>
    <wsa:To s:mustUnderstand="true">http://schemas.xmlsoap.org/ws/2004/08/addressing/role/anonymous</wsa:To>
    <wsa:ReplyTo>
      <wsa:Address>http://schemas.xmlsoap.org/ws/2004/08/addressing/role/anonymous</wsa:Address>
    </wsa:ReplyTo>
    <wsa:Action>http://schemas.xmlsoap.org/ws/2004/09/transfer/Get</wsa:Action>
    <wsa:MessageID>uuid:12345678-1234-1234-1234-1234567890ab</wsa:MessageID>
    <wsman:ResourceURI>http://schemas.microsoft.com/wbem/wsman/1/wmi/root/cimv2/Win32_OperatingSystem</wsman:ResourceURI>
    <wsman:OperationTimeout>PT60.000S</wsman:OperationTimeout>
    <wsman:Locale xml:lang="en-US" />
    <wsman:OptionSet>
      <wsman:Option Name="OptimizeEnumeration">true</wsman:Option>
    </wsman:OptionSet>
    <wsman:MaxEnvelopeSize s:mustUnderstand="true">153600</wsman:MaxEnvelopeSize>
  </s:Header>
  <s:Body />
</s:Envelope>
"#;

pub fn main() {
    let mut parsed = xml::parser::parse(SOAP).expect("Failed to parse XML");
}
